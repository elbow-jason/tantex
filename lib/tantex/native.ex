defmodule Tantex.Native do
  use Rustler, otp_app: :tantex, crate: "tantex_native"

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
  def build_schema(_field_tuples), do: err()

  def schema_into_index(_schema_ref, _path), do: err()

  def write_documents(_schema_ref, _index_ref, _list_of_docs, _heap_size), do: err()

  def limit_search(_schema_ref, _index_ref, _fields, _search_terms, _limit), do: err()

  defp err(), do: :erlang.nif_error(:nif_not_loaded)
end
