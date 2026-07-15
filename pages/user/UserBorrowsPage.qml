import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

Item {
    id: root

    ColumnLayout {
        anchors.fill: parent
        spacing: 12

        Label {
            text: qsTr("My Borrowed Books")
            font.pixelSize: 24
            font.bold: true
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
                height: 90
                color: index % 2 === 0
                       ? (Material.theme === Material.Light ? "#ffffff" : "#242724")
                       : "transparent"
                radius: 8

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 4

                    Label {
                        text: qsTr("Book ID: %1").arg(model.book_id)
                        font.bold: true
                        font.pixelSize: 15
                    }

                    RowLayout {
                        spacing: 24
                        Label {
                            text: qsTr("Borrowed: %1").arg(model.borrow_date)
                            font.pixelSize: 13
                            opacity: 0.7
                        }
                        Label {
                            text: qsTr("Due: %1").arg(model.expire_date)
                            font.pixelSize: 13
                            opacity: 0.7
                            color: !model.return_date && new Date(model.expire_date) < new Date()
                                   ? "#c62828" : undefined
                        }
                    }

                    Label {
                        text: model.return_date
                              ? qsTr("Returned: %1").arg(model.return_date)
                              : qsTr("Not returned")
                        font.pixelSize: 12
                        opacity: model.return_date ? 0.6 : 0.9
                        color: !model.return_date ? "#2e7d32" : undefined
                    }
                }
            }

            // Empty placeholder
            Label {
                anchors.centerIn: parent
                text: busy.visible ? "" : qsTr("No borrow records yet")
                opacity: 0.4
                visible: recordModel.count === 0
            }
        }
    }

    Component.onCompleted: {
        busy.visible = true
        apiClient.fetchMyBorrowRecords()
    }

    Connections {
        target: apiClient

        function onBorrowRecordsFetched(records) {
            busy.visible = false
            recordModel.clear()
            for (let i = 0; i < records.length; i++) {
                let r = records[i]
                recordModel.append({
                    id: r.id, book_id: r.book_id, user_id: r.user_id || "",
                    borrow_date: r.borrow_date, expire_date: r.expire_date,
                    return_date: r.return_date || ""
                })
            }
        }

        function onBorrowRecordsFetchFailed(msg) {
            busy.visible = false
            console.warn("Fetch failed:", msg)
        }
    }
}
