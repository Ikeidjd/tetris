Controls:<br>
    W => hold piece<br>
    S => make piece fall faster<br>
    Space => instantly drop piece<br>
    A/D => move piece<br>
    Left/Right => rotate piece

Score based on lines cleared at once:<br>
    1 => 100<br>
    2 => 300<br>
    3 => 500<br>
    4 => 800

Combos:<br>
    Each piece-drop in a row that triggers a line-clear increments combo_count<br>
    combo_count resets to 0 when a piece is dropped without triggering a line-clear<br>
    After clearing a line, you get an additional score of 50 * (combo_count - 1), calculated after incrementing combo_count (so you get no additional score on your first clear)<br>
    This is done so the text showing you your gained score is accurate even though combo_count has already been incremented