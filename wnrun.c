#include <windows.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
    if (argc < 2) {
        MessageBoxA(NULL, "Bitte eine .wn Datei angeben!\nBeispiel: wnrun test.wn", "WarumNicht", MB_ICONERROR);
        return 1;
    }

    char exePath[MAX_PATH];
    char cmd[MAX_PATH * 2];
    char fullFile[MAX_PATH];

    // Pfad zur WarumNicht.exe
    snprintf(exePath, sizeof(exePath), "C:\\Users\\%s\\AppData\\Local\\WarumNicht\\WarumNicht.exe", getenv("USERNAME"));

    // Prüfen, ob die angegebene Datei existiert
    snprintf(fullFile, sizeof(fullFile), "%s\\%s", getenv("CD"), argv[1]);
    if (GetFileAttributesA(fullFile) == INVALID_FILE_ATTRIBUTES) {
        MessageBoxA(NULL, "Die angegebene Datei wurde nicht gefunden!", "WarumNicht - Fehler", MB_ICONERROR);
        return 1;
    }

    // Befehl zusammensetzen
    snprintf(cmd, sizeof(cmd), "\"%s\" \"%s\"", exePath, fullFile);

    // Starten (ohne CMD-Fenster)
    STARTUPINFOA si = {0};
    PROCESS_INFORMATION pi = {0};
    si.cb = sizeof(si);
    si.dwFlags = STARTF_USESHOWWINDOW;
    si.wShowWindow = SW_HIDE;

    if (!CreateProcessA(NULL, cmd, NULL, NULL, FALSE, 0, NULL, NULL, &si, &pi)) {
        MessageBoxA(NULL, "Konnte WarumNicht.exe nicht starten!", "WarumNicht - Fehler", MB_ICONERROR);
        return 1;
    }

    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);
    return 0;
}
