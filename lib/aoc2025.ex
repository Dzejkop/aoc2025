defmodule Aoc2025 do
  def day1_1(f) do
    content = File.read!(f)

    turns = Day1.parse_turns(content)
    zeros = Day1.count_turns_at_zero(turns)

    IO.puts(zeros)
  end

  def day1_2(f) do
    content = File.read!(f)

    turns = Day1.parse_turns(content)
    zeros = Day1.count_zero_crossings(turns)

    IO.puts(zeros)
  end
end
