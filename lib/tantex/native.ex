defmodule Tantex.Native do
  @moduledoc false
  use Rustler, otp_app: :tantex, crate: "tantex_native"

  @type error :: {:error, {atom, String.t()}}

  @doc """

  Example:

      ```
      iex> Tantex.Native.build_schema([{"name", "text", true, false}])
      {:ok, ref}
      iex> is_reference(ref)
      true

      iex> Tantex.Native.build_schema([{"name", "texts", true, false}])
      {:error, {:invalid_type, "texts"}}

      iex> Tantex.Native.build_schema([{"name", "text", true, true}])
      {:error, {:cannot_be_fast, "text"}}

      iex> Tantex.Native.build_schema([{"name", "bytes", true, false}])
      {:error, {:cannot_be_stored, "bytes"}}

      iex> Tantex.Native.build_schema([{"", "text", true, false}])
      {:error, {:name_cannot_be_blank, ""}}
      ```
  """

  def new_schema_index(), do: err()

  def add_field(_index_ref, _field_name, _kind, _stored, _fast), do: err()

  def finalize_schema(_index_ref), do: err()

  def open_index(_index_ref, _index_path), do: err()

  def write_documents(_index_ref, _encoded_docs, _heap_size), do: err()

  def limit_search(_index_ref, _fields, _search_terms, _limit), do: err()

  def find_one_by_term(_index_ref, _field_name, _term), do: err()

  defp err(), do: :erlang.nif_error(:nif_not_loaded)
end
