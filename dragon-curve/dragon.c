#include <stdio.h>
#include <raylib.h>
#include <raymath.h>

#define linesSize 90000
#define lineSize 2

typedef struct {
  Vector2 p1;
  Vector2 p2;
} Line;

void DrawLines(Line lines[], int s) {
  for(int i = 0; i < s; i++) {
    Line l = lines[i];
    DrawLine(l.p1.x, l.p1.y, l.p2.x, l.p2.y, WHITE);
  }
}

Vector2 rotate(Vector2 p, Vector2 rp) {
  Vector2 d = Vector2Subtract(p, rp);
  Vector2 r = Vector2Add(rp, (Vector2) {-d.y, d.x});

  return r;
}

int AddLines(Line lines[], int s) {
  if (s * 2 >= linesSize)
    return s;

  Vector2 rp = lines[s-1].p2;
  int newSize = s - 1;
  for(int i = s -1; i >= 0; i--) {
    Line l = lines[i];

    Vector2 p1 = rotate(l.p1, rp);
    Vector2 p2 = rotate(l.p2, rp);

    newSize++;
    lines[newSize] = (Line) { p2, p1};

  }
  return newSize + 1;
}


int main() {

  Line lines[linesSize];
  int s = 1;
  float center = 900 / 2.0;

  lines[0] = (Line) {
    (Vector2) { center, center}, 
    (Vector2) { center + lineSize, center}, 
  };

  
  InitWindow(1500, 900, "Dragon curve");

  while(!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(BLACK);
    
    if (IsKeyPressed(KEY_SPACE)) {
      s = AddLines(lines, s);
    }

    DrawLines(lines, s);
    EndDrawing();
  }

  CloseWindow();
}

