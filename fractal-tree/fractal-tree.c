#include <math.h>
#include <raylib.h>
#include <stdio.h>

#define size 800
#define prop 0.78


void line(int x1, int y1, int x2, int y2) {
  DrawLineEx(
      (Vector2) {
        x1,
        y1
      }
      , (Vector2) {
        x2,
        y2
      }, 1, BLACK);
}
void branch(int level, int angle, int x, int y, int h) {

  if (level == 23)
    return;

  float a1 = angle - 41;
  float a2 = angle + 41;

  float r1 = a1 * PI / 180;
  float r2 = a2 * PI / 180;

  float x1 = x + cos(r1) * h;
  float y1 = y + sin(r1) * h;

  float x2 = x + cos(r2) * h;
  float y2 = y + sin(r2) * h;

  line(x, y, x1, y1);
  line(x, y, x2, y2);

  branch(level + 1, a1, x1, y1, h * prop);
  branch(level + 1, a2, x2, y2, h * prop);
}

int main() {
  int center = size / 2;
  int h = 110;
  int offset = 100;
  InitWindow(size, size, "Fractal Tree");

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(WHITE);

    line(center, size - offset, center, size - h - offset);
    branch(2, 270, center, size - offset - h, h * prop);
    EndDrawing();
  }
  CloseWindow();
}
