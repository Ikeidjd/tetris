Controls:<br>
&emsp;W => hold piece<br>
&emsp;S => make piece fall faster<br>
&emsp;Space => instantly drop piece<br>
&emsp;A/D => move piece<br>
&emsp;Left/Right => rotate piece

Score based on lines cleared at once:<br>
&emsp;1 => 100<br>
&emsp;2 => 300<br>
&emsp;3 => 500<br>
&emsp;4 => 800

Combos:<br>
&emsp;Each piece-drop in a row that triggers a line-clear increments combo_count<br>
&emsp;combo_count resets to 0 when a piece is dropped without triggering a line-clear<br>
&emsp;After clearing a line, you get an additional score of 50 * (combo_count - 1), calculated after incrementing combo_count (so you get no additional score on your first clear)<br>
&emsp;This is done so the text showing you your gained score is accurate even though combo_count has already been incremented