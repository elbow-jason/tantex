defmodule Tantex.JasonEncoder do
  def encode_map(map) when is_map(map) do
    Jason.encode!(map)
  end

  @spec decode_map(String.t()) :: map()
  def decode_map(json) when is_binary(json) do
    case Jason.decode(json) do
      {:ok, map} when is_map(map) ->
        map

      _ ->
        raise """
        Tantex.JasonEncoder.decode_map/1 encountered an error.
        Only JSON objects will decode successfully.
        JSON: #{inspect(json)}
        """
    end
  end
end
