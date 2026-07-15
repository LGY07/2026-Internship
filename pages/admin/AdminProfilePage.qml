import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

Item {
    id: root
    property int currentUserId: -1

    ColumnLayout {
        anchors.centerIn: parent
        spacing: 16
        width: Math.min(parent.width * 0.85, 420)

        Label {
            text: qsTr("Profile")
            font.pixelSize: 24
            font.bold: true
            Layout.alignment: Qt.AlignHCenter
        }

        Label {
            id: userInfoLabel
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            font.pixelSize: 14
            opacity: 0.8
            horizontalAlignment: Text.AlignHCenter
        }

        Label { text: qsTr("Phone"); font.pixelSize: 13 }
        TextField {
            id: phoneField
            Layout.fillWidth: true
            placeholderText: qsTr("Enter phone number")
        }

        Label { text: qsTr("Email"); font.pixelSize: 13 }
        TextField {
            id: emailField
            Layout.fillWidth: true
            placeholderText: qsTr("Enter email")
        }

        Button {
            text: qsTr("Save Contact Info")
            highlighted: true
            Layout.alignment: Qt.AlignHCenter
            onClicked: {
                busy.visible = true
                apiClient.updateUserInfo(currentUserId, phoneField.text, emailField.text)
            }
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Material.theme === Material.Light ? "#e0e0e0" : "#424242"
        }

        Label {
            text: qsTr("Change Password")
            font.pixelSize: 16
            font.bold: true
        }

        TextField {
            id: newPasswordField
            Layout.fillWidth: true
            echoMode: TextInput.Password
            placeholderText: qsTr("New password")
        }

        Button {
            text: qsTr("Change Password")
            highlighted: true
            Layout.alignment: Qt.AlignHCenter
            enabled: newPasswordField.text !== ""
            onClicked: {
                busy.visible = true
                apiClient.changePassword("me", newPasswordField.text)
            }
        }

        BusyIndicator {
            id: busy
            Layout.alignment: Qt.AlignHCenter
            running: false
            visible: false
        }

        Item { Layout.fillHeight: true }
    }

    Toast { id: toast }

    Component.onCompleted: apiClient.fetchCurrentUser()

    Connections {
        target: apiClient

        function onCurrentUserFetched(user) {
            currentUserId = user.id
            userInfoLabel.text = qsTr("Username: %1").arg(user.username)
                               + "  |  " + qsTr("Borrowed: %1 books").arg(user.borrowed_books)
            phoneField.text = user.phone || ""
            emailField.text = user.email || ""
        }

        function onUserInfoUpdated() {
            busy.visible = false
            toast.success(qsTr("Profile updated"))
        }

        function onUserInfoUpdateFailed(msg) {
            busy.visible = false
            toast.error(msg)
        }

        function onPasswordChanged() {
            busy.visible = false
            newPasswordField.text = ""
            toast.success(qsTr("Password changed"))
        }

        function onPasswordChangeFailed(msg) {
            busy.visible = false
            toast.error(msg)
        }
    }
}
