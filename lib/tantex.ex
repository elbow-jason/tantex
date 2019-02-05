defmodule Tantex do
  alias Tantex.{
    Index
  }

  def open(path, fields) when is_binary(path) and is_list(fields) do
    with(
      {:ok, index} <- Index.new(fields),
      :ok <- Index.finalize_schema(index),
      {:ok, index} <- Index.open_index(index, path)
    ) do
      {:ok, index}
    else
      {:error, _} = err ->
        err
    end
  end

  defdelegate find_many(index, fields, search_term, limit), to: Index
  defdelegate find_one(index, field, search_term), to: Index
  defdelegate insert_documents(index, documents, opts), to: Index
  defdelegate insert_documents(index, documents), to: Index
end
