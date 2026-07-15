import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

Item {
    id: root

    property string mode: "all"
    property string searchMode: ""

    ColumnLayout {
        anchors.fill: parent
        spacing: 12

        Label {
            text: qsTr("Borrow Records")
            font.pixelSize: 24
            font.bold: true
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: qsTr("All Records")
                highlighted: mode === "all"
                onClicked: { mode = "all"; searchMode = ""; busy.visible = true; apiClient.fetchAllBorrowRecords() }
            }
            Button {
                text: qsTr("Overdue")
                highlighted: mode === "overdue"
                onClicked: { mode = "overdue"; searchMode = ""; busy.visible = true; apiClient.fetchExpiredBorrowRecords() }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            ComboBox {
                id: userFilter
                Layout.fillWidth: true
                editable: true
                textRole: "display"
                valueRole: "value"
                model: ListModel { id: userFilterModel }
            }
            Button {
                text: qsTr("By User")
                highlighted: searchMode === "user"
                onClicked: {
                    if (userFilter.editText) {
                        searchMode = "user"
                        busy.visible = true
                        apiClient.fetchUserBorrowRecords(
                            userFilter.currentValue !== undefined
                                ? userFilter.currentValue : parseInt(userFilter.editText))
                    }
                }
            }
            ComboBox {
                id: bookFilter
                Layout.fillWidth: true
                editable: true
                textRole: "display"
                valueRole: "value"
                model: ListModel { id: bookFilterModel }
            }
            Button {
                text: qsTr("By Book")
                highlighted: searchMode === "book"
                onClicked: {
                    if (bookFilter.editText) {
                        searchMode = "book"
                        busy.visible = true
                        apiClient.fetchBookBorrowRecords(
                            bookFilter.currentValue !== undefined
                                ? bookFilter.currentValue : parseInt(bookFilter.editText))
                    }
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
            id: recordList
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            model: ListModel { id: recordModel }

            delegate: Rectangle {
                width: recordList.width
                height: 72
                color: index % 2 === 0
                       ? (Material.theme === Material.Light ? "#ffffff" : "#242724")
                       : "transparent"
                radius: 8

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 16

                    Label {
                        text: "#" + model.id
                        font.bold: true
                        font.pixelSize: 14
                    }
                    Label {
                        text: qsTr("Book %1").arg(model.book_id)
                        font.pixelSize: 13
                    }
                    Label {
                        text: qsTr("User %1").arg(model.user_id)
                        font.pixelSize: 13
                    }
                    Label {
                        text: model.borrow_date + " → " + model.expire_date
                        font.pixelSize: 12
                        opacity: 0.7
                        Layout.fillWidth: true
                    }
                    Label {
                        text: model.return_date || qsTr("Not returned")
                        color: model.return_date ? "#2e7d32" : "#f57c00"
                        font.bold: true
                        font.pixelSize: 12
                    }
                }
            }

            Label {
                anchors.centerIn: parent
                text: qsTr("No records")
                opacity: 0.4
                visible: recordModel.count === 0 && !busy.visible
            }
        }
    }

    Component.onCompleted: { apiClient.fetchAllUsers(); apiClient.fetchBooks(1) }

    Connections {
        target: apiClient

        function onAllUsersFetched(users) {
            userFilterModel.clear()
            for (let i = 0; i < users.length; i++)
                userFilterModel.append({ value: users[i].id, display: users[i].username + " (ID:" + users[i].id + ")" })
        }

        function onBooksFetched(books) {
            bookFilterModel.clear()
            for (let i = 0; i < books.length; i++) {
                let b = books[i]
                bookFilterModel.append({ value: b.id, display: b.title + " (ID:" + b.id + ")" })
            }
        }

        function onBorrowRecordsFetched(records) {
            busy.visible = false
            recordModel.clear()
            var now = new Date()
            for (let i = 0; i < records.length; i++) {
                let r = records[i]
                // When viewing "Overdue", only show expired + unreturned
                if (mode === "overdue" && (r.return_date || new Date(r.expire_date) >= now))
                    continue
                recordModel.append({
                    id: r.id, book_id: r.book_id, user_id: r.user_id,
                    borrow_date: r.borrow_date, expire_date: r.expire_date,
                    return_date: r.return_date || ""
                })
            }
        }

        function onBorrowRecordsFetchFailed(msg) {
            busy.visible = false
            console.warn(msg)
        }
    }
}
