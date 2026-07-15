import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

Item {
    id: root

    ColumnLayout {
        anchors.fill: parent
        spacing: 12

        Label {
            text: qsTr("Review Renewals")
            font.pixelSize: 24
            font.bold: true
        }

        ListView {
            id: renewalList
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            model: ListModel { id: renewalModel }

            delegate: Rectangle {
                width: renewalList.width
                height: 90
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
                        spacing: 4

                        Label {
                            text: qsTr("Renewal #%1 — Book %2 — User %3")
                                      .arg(model.id).arg(model.book_id).arg(model.user_id)
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
                            text: qsTr("Pending")
                            color: "#f57c00"
                            font.bold: true
                            font.pixelSize: 12
                        }
                    }

                    RowLayout {
                        spacing: 8
                        Button {
                            text: qsTr("Approve")
                            highlighted: true
                            onClicked: {
                                busy.visible = true
                                apiClient.approveRenewal(model.id, true)
                            }
                        }
                        Button {
                            text: qsTr("Reject")
                            Material.accent: "#c62828"
                            onClicked: {
                                busy.visible = true
                                apiClient.approveRenewal(model.id, false)
                            }
                        }
                    }
                }
            }

            Label {
                anchors.centerIn: parent
                text: qsTr("No pending renewals")
                opacity: 0.4
                visible: renewalModel.count === 0 && !busy.visible
            }
        }

        BusyIndicator {
            id: busy
            Layout.alignment: Qt.AlignHCenter
            running: false
            visible: false
        }
    }

    Toast { id: toast }

    Component.onCompleted: apiClient.fetchRenewals()

    Connections {
        target: apiClient

        function onRenewalsFetched(renewals) {
            renewalModel.clear()
            for (let i = 0; i < renewals.length; i++) renewalModel.append(renewals[i])
        }

        function onRenewalApproved() {
            busy.visible = false
            toast.success(qsTr("Done"))
            apiClient.fetchRenewals()
        }

        function onRenewalApproveFailed(msg) {
            busy.visible = false
            toast.error(msg)
        }
    }
}
