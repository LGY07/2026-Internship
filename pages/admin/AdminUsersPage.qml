import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

Item {
    id: root

    ColumnLayout {
        anchors.fill: parent
        spacing: 12

        Label {
            text: qsTr("User Management")
            font.pixelSize: 24
            font.bold: true
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            TextField {
                id: searchField
                Layout.fillWidth: true
                placeholderText: qsTr("Search by username…")
                onAccepted: {
                    if (text.trim()) {
                        busy.visible = true
                        apiClient.searchUsers(text.trim())
                    }
                }
            }

            Button {
                text: qsTr("Search")
                highlighted: true
                enabled: searchField.text.trim() !== ""
                onClicked: {
                    busy.visible = true
                    apiClient.searchUsers(searchField.text.trim())
                }
            }

            Button {
                text: qsTr("All Users")
                onClicked: {
                    busy.visible = true
                    apiClient.fetchAllUsers()
                }
            }
        }

        BusyIndicator {
            id: busy
            Layout.alignment: Qt.AlignHCenter
            running: false
            visible: false
        }

        ListView {
            id: userList
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            model: ListModel { id: userModel }

            delegate: Rectangle {
                width: userList.width
                height: 72
                color: index % 2 === 0
                       ? (Material.theme === Material.Light ? "#ffffff" : "#242724")
                       : "transparent"
                radius: 8

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 16

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 2
                        Label {
                            text: model.username
                            font.bold: true
                            font.pixelSize: 15
                        }
                        Label {
                            text: qsTr("ID: %1  |  Borrowed: %2  |  Phone: %3  |  Email: %4")
                                      .arg(model.id).arg(model.borrowed_books)
                                      .arg(model.phone || "-").arg(model.email || "-")
                            font.pixelSize: 12
                            opacity: 0.7
                        }
                    }

                    Button {
                        text: qsTr("Delete")
                        Layout.alignment: Qt.AlignRight
                        Material.accent: "#c62828"
                        onClicked: {
                            busy.visible = true
                            apiClient.removeUser(model.id)
                        }
                    }
                }
            }

            Label {
                anchors.centerIn: parent
                text: qsTr("No users found")
                opacity: 0.4
                visible: userModel.count === 0 && !busy.visible
            }
        }
    }

    Toast { id: toast }

    Connections {
        target: apiClient

        function onAllUsersFetched(users) { updateList(users) }
        function onUsersFound(users) { updateList(users) }

        function onAllUsersFetchFailed(msg) { fail(msg) }
        function onUsersSearchFailed(msg) { fail(msg) }

        function onUserRemoved() {
            busy.visible = false
            toast.success(qsTr("User deleted"))
            apiClient.fetchAllUsers()
        }

        function onUserRemoveFailed(msg) {
            busy.visible = false
            toast.error(msg)
        }

        function updateList(users) {
            busy.visible = false
            userModel.clear()
            for (let i = 0; i < users.length; i++) {
                let u = users[i]
                userModel.append({
                    id: u.id, username: u.username,
                    borrowed_books: u.borrowed_books,
                    phone: u.phone || "", email: u.email || ""
                })
            }
        }

        function fail(msg) {
            busy.visible = false
            toast.error(msg)
        }
    }
}
