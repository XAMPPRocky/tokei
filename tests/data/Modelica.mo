// 21 lines 13 code 5 comments 3 blanks
block Add "Output the sum of the two inputs"
  extends Interfaces.SI2SO;

/* 
parameter section
*/
  parameter Real k1=+1 "Gain of input signal 1";
  parameter Real k2=+1 "Gain of input signal 2";

// equation section
equation 
  y = k1*u1 + k2*u2;
  annotation (
    Documentation(info="<html>
<p>
Some documentation.
</p>
</html>"));

end Add;
