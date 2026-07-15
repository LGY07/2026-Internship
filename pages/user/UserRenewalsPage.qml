import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

Item {
    id: root

    ColumnLayout {
        anchors.fill: parent
        spacing: 12

        Label {
            text: qsTr("Renewal Requests")
            font.pixelSize: 24
            font.bold: true
        }

        Label {
            text: qsTr("Select a borrowed book to renew:")
            font.pixelSize: 13
            opacity: 0.7
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            ComboBox {
                id: borrowCombo
                Layout.fillWidth: true
                textRole: "display"
                valueRole: "id"
                model: ListModel { id: comboModel }
            }

            Label { text: qsTr("New due date:"); font.pixelSize: 13 }

            TextField {
                id: newDateField
                Layout.preferredWidth: 160
                placeholderText: "YYYY-MM-DD"
                inputMask: "0000-00-00"
            }

            Button {
                text: qsTr("Submit")
                highlighted: true
                enabled: borrowCombo.currentIndex >= 0 && newDateField.acceptableInput
                onClicked: {
                    busy.visible = true
                    apiClient.submitRenewal(borrowCombo.currentValue, newDateField.text)
                }
            }
        }

        BusyIndicator {
            id: busy
            Layout.alignment: Qt.AlignHCenter
            running: false
            visible: false
        }

        Label {
            text: qsTr("My Renewals")
            font.pixelSize: 16
            font.bold: true
            Layout.topMargin: 8
        }

        ListView {
            id: renewalList
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            model: ListModel { id: renewalModel }

            delegate: Rectangle {
                width: renewalList.width
                height: 82
                color: index % 2 === 0
                       ? (Material.theme === Material.Light ? "#ffffff" : "#242724")
                       : "transparent"
                radius: 8

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 4

                    Label {
                        text: qsTr("Renewal #%1 — Book %2").arg(model.id).arg(model.book_id)
                        font.bold: true
                        font.pixelSize: 14
                    }
                    RowLayout {
                        spacing: 16
                        Label {
                            text: qsTr("Requested: %1").arg(model.request_date)
                            font.pixelSize: 12
                            opacity: 0.7
                        }
                        Label {
                            text: qsTr("New due: %1").arg(model.expired_after)
                            font.pixelSize: 12
                            opacity: 0.7
                        }
                    }
                    Label {
                        text: model.status === "approved" ? qsTr("Approved")
                              : model.status === "rejected" ? qsTr("Rejected")
                              : qsTr("Pending")
                        color: model.status === "approved" ? "#2e7d32"
                               : model.status === "rejected" ? "#c62828"
                               : "#f57c00"
                        font.bold: true
                        font.pixelSize: 12
                    }
                }
            }

            Label {
                anchors.centerIn: parent
                text: qsTr("No renewal requests yet")
                opacity: 0.4
                visible: renewalModel.count === 0 && !busy.visible
            }
        }
    }

    Toast { id: toast }

    Component.onCompleted: {
        apiClient.fetchMyBorrowRecords()
        apiClient.fetchRenewals()
    }

    Connections {
        target: apiClient

        function onBorrowRecordsFetched(records) {
            comboModel.clear()
            for (let i = 0; i < records.length; i++) {
                let r = records[i]
                if (!r.return_date) {
                    comboModel.append({
                        id: r.id,
                        display: qsTr("Book %1 — due %2").arg(r.book_id).arg(r.expire_date)
                    })
                }
            }
        }

        function onRenewalSubmitted() {
            busy.visible = false
            toast.success(qsTr("Renewal submitted!"))
            newDateField.text = ""
            apiClient.fetchRenewals()
        }

        function onRenewalSubmitFailed(msg) {
            busy.visible = false
            toast.error(msg)
        }

        function onRenewalsFetched(renewals) {
            renewalModel.clear()
            for (let i = 0; i < renewals.length; i++) renewalModel.append(renewals[i])
        }
    }
}
