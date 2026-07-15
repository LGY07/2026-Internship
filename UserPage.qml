import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

Item {
    id: userPage

    property bool lightMode
    property int    currentIndex: 0
    signal logoutRequested()
    signal themeToggled()

    RowLayout {
        anchors.fill: parent
        spacing: 0

        // ── Sidebar ────────────────────────────────────────────
        Rectangle {
            Layout.preferredWidth: 240
            Layout.fillHeight: true
            color: Material.theme === Material.Light ? "#F9FBF9" : "#141714"

            ColumnLayout {
                anchors.fill: parent
                anchors.leftMargin: 12
                anchors.rightMargin: 12
                anchors.topMargin: 20
                anchors.bottomMargin: 12
                spacing: 0

                Label {
                    text: qsTr("Library")
                    font.pixelSize: 22
                    font.bold: true
                    Layout.leftMargin: 12
                    Layout.bottomMargin: 20
                }

                SidebarItem { emoji: "\u{1F4DA}"; title: qsTr("Search Books"); active: currentIndex === 0; onClicked: currentIndex = 0 }
                SidebarItem { emoji: "\u{1F4D6}"; title: qsTr("My Borrows");  active: currentIndex === 1; onClicked: currentIndex = 1 }
                SidebarItem { emoji: "\u{1F501}"; title: qsTr("Renewals");     active: currentIndex === 2; onClicked: currentIndex = 2 }
                SidebarItem { emoji: "\u{1F464}"; title: qsTr("Profile");      active: currentIndex === 3; onClicked: currentIndex = 3 }

                Item { Layout.fillHeight: true }

                Button {
                    flat: true
                    Layout.alignment: Qt.AlignHCenter
                    text: userPage.lightMode
                          ? "\u{1F319}  " + qsTr("Dark")
                          : "\u{2600}\u{FE0F}  " + qsTr("Light")
                    font.pixelSize: 13
                    onClicked: userPage.themeToggled()
                }

                Button {
                    flat: true
                    Layout.alignment: Qt.AlignHCenter
                    text: "\u{1F6AA}  " + qsTr("Logout")
                    font.pixelSize: 13
                    onClicked: userPage.logoutRequested()
                }
            }
        }

        // ── Content area ───────────────────────────────────────
        StackLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.margins: 24
            currentIndex: userPage.currentIndex

            UserBooksPage    {}
            UserBorrowsPage  {}
            UserRenewalsPage {}
            UserProfilePage  {}
        }
    }

    // ── MD3 pill-shaped sidebar item ───────────────────────────
    component SidebarItem: ItemDelegate {
        id: item
        Layout.fillWidth: true
        Layout.preferredHeight: 48
        Layout.topMargin: 2
        Layout.bottomMargin: 2
        leftPadding: 16

        property bool   active: false
        property string emoji: ""
        property string title: ""

        background: Rectangle {
            radius: 28
            color: item.active
                   ? (Material.theme === Material.Light ? "#DCE5DC" : "#2A362A")
                   : "transparent"
            Behavior on color { ColorAnimation { duration: 200 } }
        }

        contentItem: RowLayout {
            spacing: 12
            Label { text: emoji; font.pixelSize: 20 }
            Label { text: title; font.pixelSize: 14; font.weight: item.active ? Font.DemiBold : Font.Normal }
        }
    }
}
