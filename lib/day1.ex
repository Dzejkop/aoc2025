defmodule Day1 do
  def count_zero_crossings(turns) do
    dial = 50

    Enum.map_reduce(turns, dial, fn turn, dial ->
      {zero_crossings_in_turn(dial, turn), turn_dial(dial, turn)}
    end)
    |> elem(0)
    |> Enum.sum()
  end

  def count_turns_at_zero(turns) do
    dial = 50

    Enum.map_reduce(turns, dial, fn turn, dial ->
      {turn_dial(dial, turn), turn_dial(dial, turn)}
    end)
    |> elem(0)
    |> Stream.filter(&(&1 == 0))
    |> Enum.count()
  end

  def zero_crossings_in_turn(dial, turn) when turn > 0 do
    div(dial + turn, 100)
  end

  def zero_crossings_in_turn(dial, turn) when turn < 0 do
    div(Integer.mod(100 - dial, 100) - turn, 100)
  end

  def turn_dial(dial, turn) do
    Integer.mod(dial + turn, 100)
  end

  def parse_turn("R" <> turn) do
    Integer.parse(turn) |> elem(0)
  end

  def parse_turn("L" <> turn) do
    v = Integer.parse(turn) |> elem(0)

    -v
  end

  def parse_turns(turns) do
    String.split(turns, "\n")
    |> Stream.map(&String.trim/1)
    |> Stream.filter(&(&1 != ""))
    |> Enum.map(&parse_turn/1)
  end
end
