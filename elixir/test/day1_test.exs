defmodule Day1Test do
  use ExUnit.Case

  test "parses turn" do
    assert Day1.parse_turn("L66") == -66
    assert Day1.parse_turn("R13") == 13
  end

  test "parses turns" do
    assert Day1.parse_turns("L68\nL30\nR48") == [-68, -30, 48]
    assert Day1.parse_turns("\nL68\nL30\nR48\n") == [-68, -30, 48]
  end

  test "turns dial" do
    assert Day1.turn_dial(0, -1) == 99
    assert Day1.turn_dial(99, 1) == 0

    assert Day1.turn_dial(50, 38) == 88
    assert Day1.turn_dial(50, 183) == 33
  end

  test "day1_example" do
    turns = Day1.parse_turns("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82")
    assert Day1.count_turns_at_zero(turns) == 3
  end

  test "zero_crossings_in_turn" do
    assert Day1.zero_crossings_in_turn(73, 130) == 2

    assert Day1.zero_crossings_in_turn(0, 20) == 0
    assert Day1.zero_crossings_in_turn(52, 48) == 1
    assert Day1.zero_crossings_in_turn(0, 100) == 1
    assert Day1.zero_crossings_in_turn(10, -20) == 1
    assert Day1.zero_crossings_in_turn(10, -120) == 2
    assert Day1.zero_crossings_in_turn(0, 120) == 1
    assert Day1.zero_crossings_in_turn(0, -120) == 1
    assert Day1.zero_crossings_in_turn(99, -100) == 1
    assert Day1.zero_crossings_in_turn(99, -300) == 3
    assert Day1.zero_crossings_in_turn(50, 1000) == 10
    assert Day1.zero_crossings_in_turn(50, -1000) == 10
  end

  test "day1_2_example" do
    turns = Day1.parse_turns("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82")
    assert Day1.count_zero_crossings(turns) == 6
  end
end
