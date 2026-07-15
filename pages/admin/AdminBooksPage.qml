import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.Material

Item {
    id: root

    ColumnLayout {
        anchors.fill: parent
        spacing: 12

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            TextField {
                id: searchField
                Layout.fillWidth: true
                placeholderText: qsTr("Search by title, author, or category…")
                onAccepted: doSearch()
            }

            Button {
                text: qsTr("Search")
                highlighted: true
                enabled: searchField.text.trim() !== ""
                onClicked: doSearch()
            }

            Button {
                text: qsTr("All")
                onClicked: {
                    busy.visible = true
                    apiClient.fetchBooks(1)
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
            id: bookList
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            model: ListModel { id: bookModel }

            delegate: Rectangle {
                width: bookList.width
                height: 80
                color: index % 2 === 0
                       ? (Material.theme === Material.Light ? "#ffffff" : "#242724")
                       : "transparent"
                radius: 8

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 12

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 4
                        Label {
                            text: model.title
                            font.bold: true
                            font.pixelSize: 15
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                        }
                        Label {
                            text: "ID: " + model.id + "  |  " + model.author
                            font.pixelSize: 13
                            opacity: 0.7
                        }
                    }

                    ColumnLayout {
                        spacing: 2
                        Layout.alignment: Qt.AlignRight
                        Label {
                            text: {
                                var cats = [];
                                if (model.category) {
                                    for (var j = 0; j < model.category.length; j++)
                                        cats.push(model.category[j]);
                                }
                                return cats.join(", ");
                            }
                            font.pixelSize: 12
                            opacity: 0.6
                            horizontalAlignment: Text.AlignRight
                            Layout.fillWidth: true
                        }
                        Label {
                            text: model.borrowed ? qsTr("Borrowed") : qsTr("Available")
                            color: model.borrowed ? "#c62828" : "#2e7d32"
                            font.pixelSize: 12
                            font.bold: true
                            horizontalAlignment: Text.AlignRight
                            Layout.fillWidth: true
                        }
                        Label {
                            text: model.borrowed && model.expire_date
                                  ? qsTr("Due %1").arg(model.expire_date) : ""
                            font.pixelSize: 11
                            opacity: 0.6
                            visible: text !== ""
                            horizontalAlignment: Text.AlignRight
                            Layout.fillWidth: true
                        }
                    }
                }
            }

            Label {
                anchors.centerIn: parent
                text: qsTr("Enter a keyword and press Search")
                opacity: 0.4
                visible: bookModel.count === 0 && !busy.visible
            }
        }
    }

    function doSearch() {
        if (searchField.text.trim() === "") return
        busy.visible = true
        apiClient.searchBooks(searchField.text, searchField.text, searchField.text)
    }

    Connections {
        target: apiClient

        function onBooksFetched(books) {
            busy.visible = false
            bookModel.clear()
            for (let i = 0; i < books.length; i++) {
                let b = books[i]
                bookModel.append({
                    id: b.id, title: b.title, author: b.author,
                    category: b.category, description: b.description || "",
                    published_date: b.published_date || "",
                    borrowed: b.borrowed, expire_date: b.expire_date || ""
                })
            }
        }

        function onBooksFetchFailed(msg) {
            busy.visible = false
            console.warn("Search failed:", msg)
        }
    }
}
