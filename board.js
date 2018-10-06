const kScale = 20;
const kScaleRatio = Math.tan(60 * Math.PI / 180);

var patterns = [
  {
    begin_1: {x: 0, y: 8, step_x: 1, step_y: -1},
    begin_2: {x: -12, y: -4, step_x: 2, step_y: 0},
  },
  {
    begin_1: {x: 0, y: 8, step_x: -1, step_y: -1},
    begin_2: {x: 12, y: -4, step_x: -2, step_y: 0},
  },
  {
    begin_1: {x: 0, y: -8, step_x: 1, step_y: 1},
    begin_2: {x: -12, y: 4, step_x: 2, step_y: 0},
  },
  {
    begin_1: {x: 0, y: -8, step_x: -1, step_y: 1},
    begin_2: {x: 12, y: 4, step_x: -2, step_y: 0},
  },
  {
    begin_1: {x: -1, y: 7, step_x: -1, step_y: -1},
    begin_2: {x: 1, y: 7, step_x: 1, step_y: -1},
  },
  {
    begin_1: {x: -1, y: -7, step_x: -1, step_y: 1},
    begin_2: {x: 1, y: -7, step_x: 1, step_y: 1},
  },
];

document.addEventListener("DOMContentLoaded", function(event) {
  var canvas = document.getElementById("board");
  var dpr = window.devicePixelRatio || 1;
  var rect = canvas.getBoundingClientRect();
  canvas.width = rect.width * dpr;
  canvas.height = rect.height * dpr;

  var ctx = canvas.getContext('2d');
  ctx.scale(dpr, dpr);

  for (pattern of patterns) {
    var x1 = pattern.begin_1.x;
    var y1 = pattern.begin_1.y;
    var x2 = pattern.begin_2.x;
    var y2 = pattern.begin_2.y;
    for (i = 0; i < 12; ++i) {
      ctx.save();

      ctx.translate(rect.width / 2, rect.height / 2);
      ctx.scale(kScale, -kScale * kScaleRatio);
      ctx.moveTo(x1, y1);
      ctx.lineTo(x2, y2);

      ctx.restore();
      ctx.stroke();

      x1 += pattern.begin_1.step_x;
      y1 += pattern.begin_1.step_y;
      x2 += pattern.begin_2.step_x;
      y2 += pattern.begin_2.step_y;
    }
  }
});
