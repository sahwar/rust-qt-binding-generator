import QtQuick 2.6
import QtQml.Models 2.2
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import org.kde.kirigami 2.0 as Kirigami

ListView {
    id: view
    property string title
    header: Column {
        width: parent.width
        ToolBar {
            width: parent.width
            RowLayout {
                anchors.fill: parent
                ToolButton {
                    text: qsTr("‹")
                    enabled: dirModel.rootIndex.valid
                    onClicked: {
                        dirModel.rootIndex = dirModel.rootIndex.parent
                    }
                }
                Kirigami.Heading {
                    text: view.title
                    elide: Label.ElideMiddle
                    horizontalAlignment: Qt.AlignHCenter
                    verticalAlignment: Qt.AlignVCenter
                    Layout.fillWidth: true
                }
            }
        }
        Row {
            Text {
                width: 200
                text: qsTr("Name")
            }
            Text {
                text: qsTr("Size")
            }
        }
    }
    model: DelegateModel {
        id: dirModel
        model: sortedFileSystem
        onRootIndexChanged: {
            var index = sortedFileSystem.mapToSource(rootIndex);
            view.title = demo.fileSystemTree.filePath(index) || "";
        }
        delegate: Item {
            width: parent.width
            height: row.height
            Row {
                id: row
                Connections {
                    target: sortedFileSystem
                    onRowsInserted: {
                        // enable the button if children were found when 'model'
                        // was created or if they were just inserted
                        button.enabled = model.hasModelChildren
                                || dirModel.modelIndex(index) === parent
                    }
                }
                Button {
                    id: button
                    width: 200
                    text: fileName
                    enabled: model.hasModelChildren
                    onClicked: {
                        view.model.rootIndex = view.model.modelIndex(index)
                    }
                }
                Label {
                    text: fileSize
                    padding: button.padding
                }
            }
        }
    }
}
