#ifndef FUNCS_H
#define FUNCS_H
#include "raylib.h"

typedef struct {
    char key[50];
    char value[50];
} UserMachinePair;

typedef struct {
    Rectangle bounds;
    const char *text;
    Color colorNormal;
    Color colorHover;
    Color colorClick;
    bool isHovered;
    bool isPressed;
    bool isLoading;
} Button;

int loadUserMachinePairs(const char *filename, UserMachinePair *data, int maxEntries);
void updateLineGlow(float *glowAlpha, float glowSpeed, bool *glowIncreasing);
bool DrawButton(Button *button);
void DrawLoader(float angle, Vector2 position, float radius);
void updatePoints(int dataCount, Vector2 *startPoints, Vector2 *endPoints, bool *moveStartPoint, bool *moveEndPoint, int screenWidth);

#endif