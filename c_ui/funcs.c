#include <stdio.h>
#include <string.h>
#include "raylib.h"
#include "funcs.h"
#include <math.h>
#include <stdbool.h>

int loadUserMachinePairs(const char *filename, UserMachinePair *data, int maxEntries) {
    FILE *file = fopen(filename, "r");
    if (file == NULL) {
        perror("Error opening file");
        return -1;
    }
    int i = 0;
    char line[100]; // Line buffer

    while (fgets(line, sizeof(line), file) && i < maxEntries) {
        // Remove newline character if present
        line[strcspn(line, "\n")] = 0;

        // Split line into key and value using comma as delimiter
        char *key = strtok(line, ",");
        char *value = strtok(NULL, ",");

        if (key && value) {
            strcpy(data[i].key, key);
            strcpy(data[i].value, value);
            i++;
        }
    }
    fclose(file);
    return i;
}

void updateLineGlow(float *glowAlpha, float glowSpeed, bool *glowIncreasing){
    // glow update
    if (*glowIncreasing) {
        *glowAlpha += glowSpeed * GetFrameTime();
        if(*glowAlpha >= 1.0f) { 
            *glowIncreasing = false; 
        }
    } else {
        *glowAlpha -= glowSpeed * GetFrameTime();
        if(*glowAlpha <= 0.0f) { 
            *glowIncreasing = true; 
        }
    }
    
}

bool DrawButton(Button *button) {
    Vector2 mousePoint = GetMousePosition();
    button->isHovered = CheckCollisionPointRec(mousePoint, button->bounds);
    button->isPressed = button->isHovered && IsMouseButtonDown(MOUSE_LEFT_BUTTON);

    // Draw the button with the appropriate color based on its state
    if (button->isPressed) {
        const char* text = "Fetching Data\n\n\n\nPlease Wait...";
        int textWidth = MeasureText(text, 50);
        int textHeight = 50;
        int screenWidth = GetScreenWidth();
        int screenHeight = GetScreenHeight();
        int x = (screenWidth - textWidth) / 2;
        int y = (screenHeight - textHeight) / 2;
        button->text = "Wait";
        DrawRectangleRec(button->bounds, button->colorClick);
        DrawText(text, x, y, 50, WHITE);
    } else if (button->isHovered) {
        DrawRectangleRec(button->bounds, button->colorHover);
    } else {
        DrawRectangleRec(button->bounds, button->colorNormal);
    }

    // Draw the text centered on the button
    int textWidth = MeasureText(button->text, 20);
    int textX = button->bounds.x + (button->bounds.width - textWidth) / 2;
    int textY = button->bounds.y + (button->bounds.height - 20) / 2;
    DrawText(button->text, textX, textY, 20, RAYWHITE);

    // Return true if the button is clicked
    return (button->isHovered && IsMouseButtonReleased(MOUSE_LEFT_BUTTON));
}

void DrawLoader(float angle, Vector2 position, float radius) {
    // Draw a simple rotating loader
    DrawCircleLines(position.x, position.y, radius, DARKBLUE);
    DrawLineEx((Vector2){position.x, position.y}, 
               (Vector2){position.x + cosf(angle) * radius, position.y + sinf(angle) * radius}, 4.0f, GREEN);
}

void updatePoints(int dataCount, Vector2 *startPoints, Vector2 *endPoints, 
                    bool *moveStartPoint, bool *moveEndPoint, int screenWidth){
    for (int i = 0; i < dataCount; i++) {
        startPoints[i] = (Vector2){ (float)screenWidth / 4, 80 + i * 100 };
        endPoints[i] = (Vector2){ startPoints[i].x + (float)screenWidth / 2, startPoints[i].y };
        moveStartPoint[i] = false;
        moveEndPoint[i] = false;
        }
}