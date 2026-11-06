
#include <math.h>
#include <raylib.h>

#define SIZE 800

void circle(float x, float y, float r) {
    DrawCircleLinesEx(
        (Vector2){
          x = x,
          y = y
        },
        r,
        6,
        BLACK
    );
}

void circles(float inita, float a, float s, float r) {

  float angle = inita;
  float center = SIZE * 0.5;

  while(angle <= 360.0 + inita) {

    float x = center + r * sin(angle);
    float y = center + r * cos(angle);

    circle(x, y, s / 2.0);

    angle += a;
  }
}

int main() {
  float radius = (SIZE * 0.9) / 2.0; 
  float center = SIZE * 0.5;

  InitWindow(SIZE, SIZE, "Mandala");

  while(!WindowShouldClose()) {

    BeginDrawing();
    ClearBackground(WHITE);

    circle(center, center, radius);
    circle(center, center, radius - 20);
    circle(center, center, radius * 0.65);

    circles(0, (PI* 2.0) /8.0, 180.0, radius * 0.65); 
    circles(0, (PI* 2.0) /8.0, 120.0, radius * 0.65); 
    circles(0, (PI* 2.0) /8.0, 60.0, radius * 0.65); 

    float start = (PI*2.0) /16.0;
    circles(start, (PI*2.0) /8.0, 30.0, radius * 0.8); 
    circles(start, (PI*2.0) /8.0, 60.0, radius * 0.8); 

    circles(0.0, (PI * 2 / 8.0), 130, radius * 0.21);


    EndDrawing();
  }

  CloseWindow();
}
