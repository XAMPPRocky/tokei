//! 34 lines 15 code 16 comments 3 blanks
// https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Commented_Example_Projects
// The idea is to twist a translated circle:
// -
/*
	linear_extrude(height = 10, twist = 360, scale = 0)
	translate([1,0])
	circle(r = 1);
*/

module horn(height = 10, radius = 6,
			twist = 720, $fn = 50)
{
	// A centered circle translated by 1xR and
	// twisted by 360° degrees, covers a 2x(2xR) space.
	// -
	radius = radius/4;
	// De-translate.
	// -
	translate([-radius,0])
	// The actual code.
	// -
	linear_extrude(height = height, twist = twist,
				   scale=0, $fn = $fn)
	translate([radius,0])
	circle(r=radius);
}

translate([3,0])
mirror()
horn();

translate([-3,0])
horn();
