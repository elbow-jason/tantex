defmodule Tantex.JasonEncoder do
  @spec encode_map(map()) :: binary()
  def encode_map(map) when is_map(map) do
    Jason.encode!(map)
  end

  @spec decode_map(binary()) :: map()
  def decode_map(json) when is_binary(json) do
    case Jason.decode(json) do
      {:ok, map} when is_map(map) ->
        map

      _ ->
        raise """
        Tantex.JasonEncoder.decode_map/1 encountered an error.
        Only a JSON object that decodes into an Elixir Map (%{}) will decode successfully.
        JSON: #{inspect(json)}
        """
    end
  end
end
