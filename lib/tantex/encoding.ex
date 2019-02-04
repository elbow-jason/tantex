defmodule Tantex.Encoding do
  def encode_term(term) do
    term
    |> :erlang.term_to_binary()
    |> :erlang.binary_to_list()
  end
end
