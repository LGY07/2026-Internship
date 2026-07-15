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
            text: qsTr("Borrow a Book")
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

            RowLayout {
                spacing: 12
                Label { text: qsTr("Due date"); font.pixelSize: 13; Layout.preferredWidth: 80 }
                TextField {
                    id: expireField
                    Layout.preferredWidth: 180
                    placeholderText: "YYYY-MM-DD"
                    inputMask: "0000-00-00"
                }
            }
        }

        Button {
            text: qsTr("Borrow")
            highlighted: true
            enabled: userCombo.editText && bookCombo.editText && expireField.acceptableInput
            onClicked: {
                busy.visible = true
                apiClient.borrowBook(getComboValue(bookCombo),
                                     getComboValue(userCombo), expireField.text)
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
                if (!b.borrowed)
                    bookModel.append({ value: b.id, display: b.title + " (ID:" + b.id + ")" })
            }
        }

        function onBookBorrowed() {
            busy.visible = false
            toast.success(qsTr("Book borrowed"))
            expireField.text = ""
            apiClient.fetchBooks(1)
        }

        function onBookBorrowFailed(msg) {
            busy.visible = false
            toast.error(msg)
        }
    }
}
