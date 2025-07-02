Controls:<br>
&ensp;W => hold piece<br>
&ensp;S => make piece fall faster<br>
&ensp;Space => instantly drop piece<br>
&ensp;A/D => move piece<br>
&ensp;Left/Right => rotate piece

Score based on lines cleared at once:<br>
&ensp;1 => 100<br>
&ensp;2 => 300<br>
&ensp;3 => 500<br>
&ensp;4 => 800

Combos:<br>
&ensp;Each piece-drop in a row that triggers a line-clear increments combo_count<br>
&ensp;combo_count resets to 0 when a piece is dropped without triggering a line-clear<br>
&ensp;After clearing a line, you get an additional score of 50 * (combo_count - 1), calculated after incrementing combo_count (so you get no additional score on your first clear)<br>
&ensp;This is done so the text showing you your gained score is accurate even though combo_count has already been incremented