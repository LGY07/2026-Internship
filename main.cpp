#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QTranslator>
#include <QLocale>
#include <QQuickStyle>

#include "ApiClient.h"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    // ── Style ──────────────────────────────────────────────────────
    QQuickStyle::setStyle(QStringLiteral("Material"));

    // ── Internationalization ───────────────────────────────────────
    QTranslator translator;
    // Match the .qm files generated from i18n/*.ts
    const QStringList uiLanguages = QLocale().uiLanguages();
    for (const QString &locale : uiLanguages) {
        const QString baseName = QStringLiteral("LibraryManager_") + QLocale(locale).name();
        if (translator.load(QStringLiteral(":/i18n/") + baseName)) {
            app.installTranslator(&translator);
            break;
        }
        // Also try the two-letter code
        const QString shortName = QStringLiteral("LibraryManager_")
                                  + QLocale(locale).name().left(2);
        if (translator.load(QStringLiteral(":/i18n/") + shortName)) {
            app.installTranslator(&translator);
            break;
        }
    }

    // ── API Client ─────────────────────────────────────────────────
    ApiClient apiClient;
    // Set the base URL to your server (HTTPS as required by the API spec)
    apiClient.setBaseUrl(QStringLiteral("https://aphanite.rosmontis.edu.kg"));

    // ── QML Engine ─────────────────────────────────────────────────
    QQmlApplicationEngine engine;

    // Expose ApiClient to QML
    engine.rootContext()->setContextProperty(QStringLiteral("apiClient"), &apiClient);

    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreationFailed,
        &app,
        []() { QCoreApplication::exit(-1); },
        Qt::QueuedConnection);

    engine.loadFromModule("LibraryManager", "Main");

    return app.exec();
}
