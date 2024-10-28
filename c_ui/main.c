#include "raylib.h"
#include <stdio.h>
#include <stdlib.h>
#include "funcs.h"

// #ifndef _DEBUG
// #pragma comment(linker, "/SUBSYSTEM:windows /ENTRY:mainCRTStartup")
// #endif

int main(void) {
    SetTraceLogLevel(LOG_NONE);
    // get live data at the beginning
    system("nct.exe --get-live-ui");
    int screenWidth = 1200;
    int screenHeight = 800;
    const int textureSize = 70;
    float scrollOffset = 0;
    const float scrollSpeed = 10;
    // consts for glow
    float glowAlpha = 0.0f;
    float glowSpeed = 1.0f;
    bool glowIncreasing = true;
    // consts for circle around profile 
    float profileCircleRotation = 0.0f;
    // count down
    int startTime = 60;
    int currentTime = startTime;
    double timer = 0.0;
    // button
    float loaderAngle = 0.0f;
    float loaderTime = 0.0f;
    bool isButtonClicked = false;

    SetConfigFlags(FLAG_MSAA_4X_HINT | FLAG_WINDOW_RESIZABLE);
    InitWindow(screenWidth, screenHeight, "Connections to Remote Machines");

    // load window icon
    Image icon = LoadImage("src/cp2.png");
    SetWindowIcon(icon);

    // Load bg image
    Image bgImage = LoadImage("src/bg1.png");
    ImageResize(&bgImage, screenWidth, screenHeight);
    Texture2D bg = LoadTextureFromImage(bgImage);
    UnloadImage(bgImage);

    // Load the profile image
    Image profileImage = LoadImage("src/profile.png");
    ImageResize(&profileImage, textureSize, textureSize);
    Texture2D profileTexture = LoadTextureFromImage(profileImage);
    UnloadImage(profileImage);

    // Load circle around profile image
    Image circleProfile = LoadImage("src/cp3.png");
    ImageResize(&circleProfile, textureSize+15, textureSize+15);
    Texture2D circleProfileTexture = LoadTextureFromImage(circleProfile);
    UnloadImage(circleProfile);

    // Load the computer image
    Image pcImage = LoadImage("src/pc.png");
    ImageResize(&pcImage, textureSize, textureSize);
    Texture2D pcTexture = LoadTextureFromImage(pcImage);
    UnloadImage(pcImage);

    UserMachinePair data[100]; 
    int dataCount = loadUserMachinePairs("live_data.csv", data, 100);
    if (dataCount == -1) {
        printf("ERROR! Could not read data!");
        return 1;
    }
    Vector2 *startPoints = malloc(dataCount * sizeof(Vector2));
    Vector2 *endPoints = malloc(dataCount * sizeof(Vector2));
    bool *moveStartPoint = malloc(dataCount * sizeof(bool));
    bool *moveEndPoint = malloc(dataCount * sizeof(bool));
    if (startPoints == NULL || endPoints == NULL || moveStartPoint == NULL || moveEndPoint == NULL) {
        printf("ERROR! Could not allocate memory!");
        return 1;
    }
    // Vector2 startPoints[dataCount];
    // Vector2 endPoints[dataCount];
    // bool moveStartPoint[dataCount];
    // bool moveEndPoint[dataCount];
    updatePoints(dataCount, startPoints, endPoints, moveStartPoint, moveEndPoint, screenWidth);       

    SetTargetFPS(60);

    while (!WindowShouldClose()) {
        // Create Button 
        Button refreshButton = {
            .bounds = { screenWidth - 250, 30, 100, 50 },
            .text = "Refresh",
            .colorNormal = GREEN,
            .colorHover = DARKGREEN,
            .colorClick = GRAY,
            .isLoading = false
        };
        if (DrawButton(&refreshButton)) {
            isButtonClicked = true;
            refreshButton.isLoading = true;
            loaderTime = 0.0f;
        // Update loader rotation and time
        if (refreshButton.isLoading) {
            loaderAngle += 0.05f;    // Rotate the loader
            loaderTime += GetFrameTime();
            int result = system("nct.exe --get-live-ui");
            if (result == 0){
                refreshButton.isLoading = false;
                isButtonClicked = false;
                int dataCount = loadUserMachinePairs("live_data.csv", data, 100);
                updatePoints(dataCount, startPoints, endPoints, moveStartPoint, moveEndPoint, screenWidth);       
                currentTime = startTime;
        }
            }
        }
        timer += GetFrameTime();
        if (timer >= 1.0){
            currentTime--;
            timer = 0.0;
        }

        bool isWinSizeChanged = false;
        scrollOffset += GetMouseWheelMove() * scrollSpeed;
        Vector2 mouse = GetMousePosition();
        updateLineGlow(&glowAlpha, glowSpeed, &glowIncreasing);
        profileCircleRotation += 0.50f; // speed for rotating the circle around profile
        if (profileCircleRotation > 360.0f) { profileCircleRotation = 0.0f; }
        if (IsWindowResized()) {
            screenWidth = GetScreenWidth();
            screenHeight = GetScreenHeight();
            printf("Window is resized. %d, %d", screenWidth, screenHeight);
            bgImage = LoadImage("src/bg1.png");
            ImageResize(&bgImage, screenWidth, screenHeight);
            bg = LoadTextureFromImage(bgImage);
            UnloadImage(bgImage);
            int dataCount = loadUserMachinePairs("live_data.csv", data, 100);
            updatePoints(dataCount, startPoints, endPoints, moveStartPoint, moveEndPoint, screenWidth);       
        }
        int dataCount = loadUserMachinePairs("live_data.csv", data, 100);
        updatePoints(dataCount, startPoints, endPoints, moveStartPoint, moveEndPoint, screenWidth);       
        for (int i = 0; i < dataCount; i++) {
            Vector2 startPoint = { (float)screenWidth / 4, 80 + i * 100 + scrollOffset };
            Vector2 endPoint = { startPoint.x + (float)screenWidth / 2, startPoint.y };
            
            if (CheckCollisionPointCircle(mouse, startPoints[i], 10.0f) &&
                IsMouseButtonDown(MOUSE_BUTTON_LEFT)) moveStartPoint[i] = true;
            else if (CheckCollisionPointCircle(mouse, endPoints[i], 10.0f) && 
                IsMouseButtonDown(MOUSE_BUTTON_LEFT)) moveEndPoint[i] = true;

            if (moveStartPoint[i]) {
                startPoints[i] = mouse;
                if (IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) moveStartPoint[i] = false;
            }

            if (moveEndPoint[i]) {
                endPoints[i] = mouse;
                if (IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) moveEndPoint[i] = false;
            }
        }
        Vector2 origin = {(float)circleProfileTexture.width / 2, (float)circleProfileTexture.height / 2};

        BeginDrawing();
            ClearBackground(BLACK);
            DrawTexture(bg, 0, 0, WHITE);
            DrawText("See who is connected to which IFOS machines", 15, 20, 20, GRAY);
            // Button
            DrawButton(&refreshButton);
            // Draw the loader when the button is clicked
            // if (refreshButton.isLoading) {
            //     Vector2 loaderPosition = { screenWidth / 2, screenHeight / 2 };
            //     DrawLoader(loaderAngle, loaderPosition, 30.0f);
            // }
            // Count Down
            if (currentTime > 0){
                DrawText(TextFormat("Auto Refresh in %02d Seconds", currentTime), screenWidth - 250, 90, 15, BLUE);
            } else {
                int result = system("nct.exe --get-live-ui");
                // if (result == 0){printf("Live Data updated!\n");}
                // else {printf("Fetching Live Data Failed!\n");}
                currentTime = startTime;
            }
            for (int i = 1; i < dataCount; i++) {
                char ifos_str[15];
                snprintf(ifos_str, sizeof(ifos_str), "%s%s", "IFOS-TE", data[i].value);
                int textWidth = MeasureText(data[i].key, 20);
                DrawLineBezier(startPoints[i], endPoints[i], 2.0f, GRAY);
                DrawLineEx(startPoints[i], endPoints[i], 6.0f, Fade(RED, glowAlpha));
                DrawText(data[i].key, (int)startPoints[i].x - textWidth - 50, (int)startPoints[i].y - 10, 20, BROWN);
                DrawText(ifos_str, (int)endPoints[i].x + 40, (int)endPoints[i].y - 10, 20, BROWN);
                DrawTexture(profileTexture, (int)startPoints[i].x - profileTexture.width / 2, (int)startPoints[i].y - profileTexture.height / 2, WHITE);
                DrawTexture(pcTexture, (int)endPoints[i].x - pcTexture.width / 2, (int)endPoints[i].y - pcTexture.height / 2, WHITE);
                DrawTexturePro(
                    circleProfileTexture,// Texture
                    (Rectangle){0, 0, (float)circleProfileTexture.width, 
                            (float)circleProfileTexture.height},  // Source rectangle
                    (Rectangle){startPoints[i].x, startPoints[i].y, 
                            (float)circleProfileTexture.width, (float)circleProfileTexture.height},  // Destination rectangle
                    origin,                                       
                    profileCircleRotation, // Rotation angle
                    WHITE// Tint color
                );
            }
        EndDrawing();
    }
    free(startPoints);
    free(endPoints);
    free(moveStartPoint);
    free(moveEndPoint);
    // Unload the textures
    UnloadTexture(bg);
    UnloadTexture(pcTexture);
    UnloadTexture(profileTexture);
    UnloadImage(icon);
    CloseWindow();
    return 0;
}
