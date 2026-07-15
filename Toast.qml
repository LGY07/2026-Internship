import QtQuick
import QtQuick.Controls.Material

Item {
    id: root
    anchors.fill: parent
    z: 999

    property string message: ""
    property bool   isError: false

    function show(text, error) {
        message = text
        isError = error || false
        toast.opacity = 1
        dismissTimer.restart()
    }

    function success(text) { show(text, false) }
    function error(text)   { show(text, true) }

    Timer {
        id: dismissTimer
        interval: 4000
        onTriggered: toast.opacity = 0
    }

    Rectangle {
        id: toast
        anchors {
            horizontalCenter: parent.horizontalCenter
            bottom: parent.bottom
            bottomMargin: 48
        }
        width: Math.min(messageLabel.implicitWidth + 40, parent.width - 48)
        height: messageLabel.implicitHeight + 24
        radius: 20
        opacity: 0
        color: Material.theme === Material.Light
               ? (isError ? "#BA1A1A" : "#2E2E2E")
               : (isError ? "#FFB4AB" : "#E0E0E0")

        Behavior on opacity { NumberAnimation { duration: 250 } }

        Label {
            id: messageLabel
            anchors.centerIn: parent
            text: root.message
            color: Material.theme === Material.Light ? "#FFFFFF"
                   : (isError ? "#601410" : "#1C1B1F")
            font.pixelSize: 14
            horizontalAlignment: Text.AlignHCenter
        }
    }
}
