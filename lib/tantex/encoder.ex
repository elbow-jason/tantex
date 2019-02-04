defmodule Tantex.Encoder do
  @callback encode_map(map()) :: String.t()
  @callback decode_map(String.t()) :: map()

  def get_encoder() do
    Application.get_env(:tantex, :encoder, Tantex.JasonEncoder)
  end
end
