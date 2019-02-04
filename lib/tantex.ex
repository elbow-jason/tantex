defmodule Tantex do
  alias Tantex.{
    Field,
    Native,
    Index
  }

  def open(path, fields) when is_binary(path) and is_list(fields) do
    native_fields = Enum.map(fields, &Field.to_native_tuple/1)

    with(
      {:ok, schema} <- Native.build_schema(native_fields),
      {:ok, index} <- Native.schema_into_index(schema, path)
    ) do
      {:ok, %Index{index_ref: index, schema_ref: schema, fields: fields}}
    else
      {:error, _} = err ->
        err
    end
  end

  def encode_term(term) do
    term
    |> :erlang.term_to_binary()
    |> :erlang.binary_to_list()
  end
end
