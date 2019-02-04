defmodule Tantex.Field do
  alias Tantex.Field

  @type kind :: :u64 | :i64 | :bytes | :facet | :text | :string

  @kinds [
    :u64,
    :i64,
    :bytes,
    :facet,
    :text,
    :string
  ]

  @type t :: %Field{
          kind: kind(),
          name: String.t(),
          fast: boolean(),
          stored: boolean()
        }

  defstruct [:kind, :name, :fast, :stored]

  defguard is_kind(k) when k in @kinds

  @spec build(binary(), kind(), Keyword.t()) :: Field.t()
  def build(name, kind, opts \\ []) when is_kind(kind) and is_binary(name) and is_list(opts) do
    {stored, fast} = generate_stored_and_fast(kind, opts)

    %Field{
      name: name,
      kind: kind,
      stored: stored,
      fast: fast
    }
  end

  @doc false
  def to_native_tuple(%Field{
        kind: kind,
        name: name,
        stored: stored,
        fast: fast
      }) do
    {name, to_string(kind), stored, fast}
  end

  defp generate_stored_and_fast(k, opts) when k in [:i64, :u64] do
    stored = Keyword.get(opts, :stored, true)
    fast = Keyword.get(opts, :fast, true)
    {stored, fast}
  end

  defp generate_stored_and_fast(k, opts) when k in [:text, :string] do
    stored = Keyword.get(opts, :stored, true)
    {stored, false}
  end

  defp generate_stored_and_fast(:bytes, opts) do
    fast = Keyword.get(opts, :fast, true)
    {false, fast}
  end

  defp generate_stored_and_fast(:facet, _) do
    {false, false}
  end
end
