import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

Item {
    id: root

    function getComboValue(combo) {
        return combo.currentValue !== undefined ? combo.currentValue : parseInt(combo.editText)
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 16

        Label {
            text: qsTr("Return a Book")
            font.pixelSize: 24
            font.bold: true
        }

        ColumnLayout {
            spacing: 8

            RowLayout {
                spacing: 12
                Label { text: qsTr("User"); font.pixelSize: 13; Layout.preferredWidth: 80 }
                ComboBox {
                    id: userCombo
                    Layout.fillWidth: true
                    editable: true
                    textRole: "display"
                    valueRole: "value"
                    model: ListModel { id: userModel }
                }
            }

            RowLayout {
                spacing: 12
                Label { text: qsTr("Book"); font.pixelSize: 13; Layout.preferredWidth: 80 }
                ComboBox {
                    id: bookCombo
                    Layout.fillWidth: true
                    editable: true
                    textRole: "display"
                    valueRole: "value"
                    model: ListModel { id: bookModel }
                }
            }
        }

        Button {
            text: qsTr("Return")
            highlighted: true
            enabled: bookCombo.editText && userCombo.editText
            onClicked: {
                busy.visible = true
                apiClient.returnBook(getComboValue(bookCombo), getComboValue(userCombo))
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

    Component.onCompleted: { apiClient.fetchAllUsers(); apiClient.fetchBooks(1) }

    Connections {
        target: apiClient

        function onAllUsersFetched(users) {
            userModel.clear()
            for (let i = 0; i < users.length; i++)
                userModel.append({ value: users[i].id, display: users[i].username + " (ID:" + users[i].id + ")" })
        }

        function onBooksFetched(books) {
            bookModel.clear()
            for (let i = 0; i < books.length; i++) {
                let b = books[i]
                bookModel.append({ value: b.id, display: b.title + " (ID:" + b.id + ")" })
            }
        }

        function onBookReturned() {
            busy.visible = false
            toast.success(qsTr("Book returned"))
            apiClient.fetchBooks(1)
        }

        function onBookReturnFailed(msg) {
            busy.visible = false
            toast.error(msg)
        }
    }
}
