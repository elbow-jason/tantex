defmodule Tantex.Index do
  alias Tantex.{Encoder, Native, Index, Field}

  @type field_name :: atom() | binary() | Field.t()
  @type t :: %__MODULE__{
          __ref__: reference(),
          fields: list(Field.t()),
          path: nil | String.t()
        }

  defstruct __ref__: nil, fields: [], path: nil

  def new(fields \\ []) do
    with(
      {:ok, ref} <- Native.new_schema_index(),
      index <- %Index{__ref__: ref, fields: []},
      {:ok, index} <- add_fields(index, fields)
    ) do
      {:ok, index}
    end
  end

  def add_field(
        %Index{
          __ref__: ref,
          fields: prev
        } = index,
        %Field{
          name: name,
          kind: kind,
          fast: fast,
          stored: stored
        } = field
      ) do
    case Native.add_field(ref, to_string(name), to_string(kind), stored, fast) do
      :ok ->
        {:ok, %Index{index | fields: [field | prev]}}

      err ->
        err
    end
  end

  def add_fields(%Index{} = index, fields) when is_list(fields) do
    fields
    |> Enum.reduce_while(index, fn field, acc ->
      acc
      |> add_field(field)
      |> case do
        {:ok, index} -> {:cont, index}
        err -> {:halt, err}
      end
    end)
    |> case do
      {:error, _} = err -> err
      %Index{} = index -> {:ok, index}
    end
  end

  def insert_documents(%Index{__ref__: ref}, docs, opts \\ []) when is_list(docs) do
    heap_size = Keyword.get(opts, :heap_size, 50_000_000)
    encoder = Encoder.get_encoder()
    list_of_docs = Enum.map(docs, fn doc -> encoder.encode_map(doc) end)
    Native.write_documents(ref, list_of_docs, heap_size)
  end

  @spec find_many(Tantex.Index.t(), list(String.t()), String.t(), non_neg_integer()) ::
          {:ok, list(map())} | Native.error()
  def find_many(%Index{__ref__: ref}, fields, search_terms, limit) do
    fields = Enum.map(fields, &string_field/1)

    case Native.limit_search(ref, fields, search_terms, limit) do
      {:ok, json_list} ->
        encoder = Encoder.get_encoder()
        {:ok, Enum.map(json_list, fn item -> encoder.decode_map(item) end)}

      err ->
        err
    end
  end

  @spec find_one(Index.t(), field_name(), String.t()) :: {:ok, map} | Native.error()
  def find_one(%Index{__ref__: ref}, field, term) when is_binary(term) or is_integer(term) do
    case Native.find_one_by_term(ref, string_field(field), term) do
      {:ok, json_doc} ->
        {:ok, Encoder.get_encoder().decode_map(json_doc)}

      err ->
        err
    end
  end

  @spec open_index(Tantex.Index.t(), String.t()) :: {:ok, Index.t()} | Native.error()
  def open_index(%Index{__ref__: ref} = index, path) do
    case Native.open_index(ref, path) do
      :ok -> {:ok, %Index{index | path: path}}
      err -> err
    end
  end

  def finalize_schema(%Index{__ref__: ref}) do
    Native.finalize_schema(ref)
  end

  defp string_field(x) when is_binary(x), do: x
  defp string_field(x) when is_atom(x), do: to_string(x)
  defp string_field(%Field{name: name}), do: to_string(name)
end
