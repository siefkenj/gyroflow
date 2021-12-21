use qmetaobject::qrc;

qrc!(pub rsrc,
    "/" {
        "src/ui/main_window.qml",
        "src/ui/App.qml",
        "src/ui/VideoArea.qml",

        "src/ui/menu/Advanced.qml",
        "src/ui/menu/Export.qml",
        "src/ui/menu/LensProfile.qml",
        "src/ui/menu/MotionData.qml",
        "src/ui/menu/Stabilization.qml",
        "src/ui/menu/Synchronization.qml",
        "src/ui/menu/VideoInformation.qml",
        "src/ui/components/BasicText.qml",
        "src/ui/components/Button.qml",
        "src/ui/components/CheckBox.qml",
        "src/ui/components/CheckBoxWithContent.qml",
        "src/ui/components/ComboBox.qml",
        "src/ui/components/DropdownChevron.qml",
        "src/ui/components/DropTarget.qml",
        "src/ui/components/DropTargetRect.qml",
        "src/ui/components/Ease.qml",
        "src/ui/components/Hr.qml",
        "src/ui/components/Label.qml",
        "src/ui/components/LinkButton.qml",
        "src/ui/components/LoaderOverlay.qml",
        "src/ui/components/MenuItem.qml",
        "src/ui/components/Modal.qml",
        "src/ui/components/NumberField.qml",
        "src/ui/components/Popup.qml",
        "src/ui/components/Menu.qml",
        "src/ui/components/Action.qml",
        "src/ui/components/ResizablePanel.qml",
        "src/ui/components/SearchField.qml",
        "src/ui/components/SidePanel.qml",
        "src/ui/components/Slider.qml",
        "src/ui/components/SliderWithField.qml",
        "src/ui/components/SplitButton.qml",
        "src/ui/components/TableList.qml",
        "src/ui/components/TextField.qml",
        "src/ui/components/Timeline.qml",
        "src/ui/components/TimelineAxisButton.qml",
        "src/ui/components/TimelineRangeIndicator.qml",
        "src/ui/components/TimelineSyncPoint.qml",
        "src/ui/components/ToolTip.qml",
        "src/ui/components/InfoMessage.qml",
        "src/ui/components/InfoMessageSmall.qml",
        
        "resources/icon.png",
        "resources/logo_black.svg",
        "resources/logo_white.svg",
        "resources/icons/index.theme",
        "resources/icons/svg/bin.svg",
        "resources/icons/svg/chart.svg",
        "resources/icons/svg/checkmark.svg",
        "resources/icons/svg/chevron-down.svg",
        "resources/icons/svg/chevron-left.svg",
        "resources/icons/svg/chevron-right.svg",
        "resources/icons/svg/file-empty.svg",
        "resources/icons/svg/gyroflow.svg",
        "resources/icons/svg/info.svg",
        "resources/icons/svg/lens.svg",
        "resources/icons/svg/lock.svg",
        "resources/icons/svg/pause.svg",
        "resources/icons/svg/pencil.svg",
        "resources/icons/svg/play.svg",
        "resources/icons/svg/plus.svg",
        "resources/icons/svg/save.svg",
        "resources/icons/svg/search.svg",
        "resources/icons/svg/settings.svg",
        "resources/icons/svg/sound-mute.svg",
        "resources/icons/svg/sound.svg",
        "resources/icons/svg/spinner.svg",
        "resources/icons/svg/sync.svg",
        "resources/icons/svg/unlocked.svg",
        "resources/icons/svg/video.svg",
        "resources/icons/svg/confirmed.svg",
        "resources/icons/svg/error.svg",
        "resources/icons/svg/warning.svg",
        "resources/icons/svg/readout_time.svg",

        "src/qt_gpu/compiled/undistort.comp.qsb",
        "src/qt_gpu/compiled/texture.frag.qsb",
        "src/qt_gpu/compiled/texture.vert.qsb",

        "resources/translations/pl.qm",
    }
);
