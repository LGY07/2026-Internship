import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

Item {
    id: loginPage

    property bool isRegisterMode: false
    property bool isBusy: false

    signal loginRequested(string username, string password)
    signal registerRequested(string username, string password)
    signal toggleTheme()

    function showError(msg) {
        toast.error(msg)
        isBusy = false
        timeoutTimer.stop()
    }

    function showSuccess() {
        isBusy = false
        timeoutTimer.stop()
    }

    Toast { id: toast }

    Timer {
        id: timeoutTimer
        interval: 10000
        onTriggered: {
            isBusy = false
            toast.error(qsTr("Request timed out, please try again"))
        }
    }

    Pane {
        anchors.fill: parent

        background: Rectangle {
            color: Material.theme === Material.Light ? "#F4F8F4" : "#191C19"
        }

        // ── Theme toggle ──────────────────────────────────────────
        Button {
            anchors { right: parent.right; top: parent.top; margins: 12 }
            flat: true
            text: Material.theme === Material.Light ? "\u{1F319}" : "\u{2600}\u{FE0F}"
            font.pixelSize: 20
            onClicked: loginPage.toggleTheme()
        }

        ColumnLayout {
            anchors.centerIn: parent
            width: Math.min(parent.width * 0.85, 400)
            spacing: 20

            // Title
            ColumnLayout {
                Layout.alignment: Qt.AlignHCenter
                spacing: 4

                Label {
                    text: qsTr("Library Manager")
                    font.pixelSize: 32
                    font.bold: true
                    horizontalAlignment: Text.AlignHCenter
                    Layout.fillWidth: true
                }

                Label {
                    text: isRegisterMode ? qsTr("Create Account") : qsTr("Sign In")
                    font.pixelSize: 16
                    opacity: 0.7
                    horizontalAlignment: Text.AlignHCenter
                    Layout.fillWidth: true
                }
            }

            // Username field
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6

                Label {
                    text: qsTr("Username")
                    font.pixelSize: 13
                }

                TextField {
                    id: usernameField
                    Layout.fillWidth: true
                    placeholderText: qsTr("Enter username")
                    enabled: !isBusy

                    Keys.onReturnPressed: passwordField.forceActiveFocus()
                }
            }

            // Password field
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6

                Label {
                    text: qsTr("Password")
                    font.pixelSize: 13
                }

                Item {
                    Layout.fillWidth: true
                    implicitHeight: passwordField.implicitHeight

                    TextField {
                        id: passwordField
                        anchors { left: parent.left; right: parent.right }
                        echoMode: TextInput.Password
                        placeholderText: qsTr("Enter password")
                        enabled: !isBusy
                        rightPadding: 44

                        Keys.onReturnPressed: submitButton.clicked()
                    }

                    Button {
                        id: showPasswordButton
                        anchors {
                            right: passwordField.right
                            rightMargin: 4
                            verticalCenter: passwordField.verticalCenter
                        }
                        flat: true
                        width: 36
                        height: 36

                        contentItem: Text {
                            text: passwordField.echoMode === TextInput.Password ? "\u{1F441}" : "\u{1F576}"
                            font.pixelSize: 18
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }

                        onClicked: {
                            passwordField.echoMode = passwordField.echoMode === TextInput.Password
                                    ? TextInput.Normal : TextInput.Password
                        }
                    }
                }
            }

            // Submit button
            Button {
                id: submitButton
                Layout.fillWidth: true
                Layout.topMargin: 8
                text: isRegisterMode ? qsTr("Register") : qsTr("Login")
                highlighted: true
                enabled: usernameField.text.trim() !== "" && passwordField.text !== "" && !isBusy

                onClicked: {
                    isBusy = true
                    timeoutTimer.restart()
                    if (isRegisterMode) {
                        registerRequested(usernameField.text.trim(), passwordField.text)
                    } else {
                        loginRequested(usernameField.text.trim(), passwordField.text)
                    }
                }
            }

            // Loading indicator
            BusyIndicator {
                Layout.alignment: Qt.AlignHCenter
                running: isBusy
                visible: isBusy
            }

            // Switch mode
            RowLayout {
                Layout.alignment: Qt.AlignHCenter
                spacing: 4

                Label {
                    text: isRegisterMode ? qsTr("Already have an account?") : qsTr("Don't have an account?")
                    font.pixelSize: 13
                    opacity: 0.7
                }

                Button {
                    flat: true
                    enabled: !isBusy
                    text: isRegisterMode ? qsTr("Sign In") : qsTr("Register")
                    font.underline: true

                    onClicked: {
                        isRegisterMode = !isRegisterMode
                    }
                }
            }
        }
    }
}
