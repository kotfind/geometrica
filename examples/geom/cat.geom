clear!

// Mirror
mir p:pt -> pt = let
    axis = 200.0,
    x = -(p.x - axis) + axis,
    y = p.y,
in pt x y

mir l:line -> line = line (l.p1.mir) (l.p2.mir)
mir c:circ -> circ = circ (c.o.mir) (c.r)

// Eyes
pupil = pt 120.0 200.0
eye = circ pupil 20.0

eye_m = mir eye
pupil_m = mir pupil

// Head
side_head = line (pt 0.0 0.0) (pt 50.0 300.0)
jaw = line (side_head.p2) (pt 200.0 400.0)
ear = line (side_head.p1) (pt 140.0 110.0)
forehead = line (ear.p2) (pt 200.0 (ear.p2.y))
ear_sep = line (forehead.p1) (pt 23.0 140.0)

side_head_m = mir side_head
jaw_m = mir jaw
ear_m = mir ear
forehead_m = mir forehead
ear_sep_m = mir ear_sep

// Nose
nose_side = line (pt 185.0 260.0) (pt 200.0 280.0)
nose_top = line (nose_side.p1) (pt 200.0 (nose_side.p1.y))

nose_side_m = mir nose_side
nose_top_m = mir nose_top

// Whiskers
whisker1 = line (nose_side.p2 + (pt 0.0 5.0)) (pt 500.0 270.0)
whisker2 = line (nose_side.p2 + (pt 0.0 5.0)) (pt 480.0 300.0)
whisker3 = line (nose_side.p2 + (pt 0.0 5.0)) (pt 460.0 330.0)

whisker1_m = mir whisker1
whisker2_m = mir whisker2
whisker3_m = mir whisker3

// Output
save_svg! "cat.svg"
