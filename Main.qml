import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

ApplicationWindow {
    id: window
    width: 900
    height: 600
    minimumWidth: 900
    minimumHeight: 600
    visible: true
    title: qsTr("Library Manager")

    // ── Material Theme (MD3 Monet) ──────────────────────────────────
    Material.theme: lightMode ? Material.Light : Material.Dark
    Material.primary: "#006C4C"     // MD3 Monet tonal green
    Material.accent: "#709489"      // soft sage accent

    property bool lightMode: Application.styleHints.colorScheme === Qt.Light

    // ── StackView ──────────────────────────────────────────────────
    StackView {
        id: stackView
        anchors.fill: parent
        initialItem: loginComponent
    }

    // ── Login Page Component ───────────────────────────────────────
    Component {
        id: loginComponent

        LoginPage {
            onLoginRequested: function(username, password) {
                apiClient.login(username, password)
            }

            onRegisterRequested: function(username, password) {
                apiClient.registerUser(username, password)
            }

            onToggleTheme: window.lightMode = !window.lightMode
        }
    }

    // ── User / Admin components ────────────────────────────────────
    Component {
        id: userComponent
        UserPage {
            lightMode: window.lightMode
            onLogoutRequested: stackView.pop(null)
            onThemeToggled: window.lightMode = !window.lightMode
        }
    }

    Component {
        id: adminComponent
        AdminPage {
            lightMode: window.lightMode
            onLogoutRequested: stackView.pop(null)
            onThemeToggled: window.lightMode = !window.lightMode
        }
    }

    // ── API Client connections ─────────────────────────────────────
    Connections {
        target: apiClient

        function onLoginSucceeded(accessToken, permissions) {
            loginPage().showSuccess()
            var page = (permissions === "manager") ? adminComponent : userComponent
            stackView.push(page)
            console.log("Login succeeded, permissions:", permissions)
        }

        function onLoginFailed(message) {
            loginPage().showError(message)
        }

        function onRegisterSucceeded() {
            loginPage().showSuccess()
            loginPage().isRegisterMode = false
            console.log("Registration succeeded, please login")
        }

        function onRegisterFailed(message) {
            loginPage().showError(message)
        }
    }

    function loginPage() {
        return stackView.currentItem
    }
}
