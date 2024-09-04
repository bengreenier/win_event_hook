use windows::Win32::UI::WindowsAndMessaging::*;

/// A macro that creates a `TryFrom<u32>` implementation for a `repr(u32)` enum.
/// Adapted from https://stackoverflow.com/a/57578431
macro_rules! u32_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u32> for $name {
            type Error = crate::errors::Error;

            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u32 => Ok($name::$vname),)*
                    _ => Err(crate::errors::Error::InvalidEvent(v)),
                }
            }
        }
    }
}

/// Windows accessibility events.
/// See variant documentation ([`NamedEvent`],[`AiaEvent`],[`OemEvent`],[`UiaEvent`],[`UiaPropertyEvent`]) for more information.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Event {
    Named(NamedEvent),
    Aia(AiaEvent),
    Oem(OemEvent),
    Uia(UiaEvent),
    UiaProperty(UiaPropertyEvent),
    Unknown(u32),
}

impl Event {
    /// The lowest possible [`Event`] source value ([`u32`]).
    pub const MIN: u32 = EVENT_MIN;

    /// The highest possible [`Event`] source value ([`u32`]).
    pub const MAX: u32 = EVENT_MAX;
}

impl From<NamedEvent> for Event {
    fn from(value: NamedEvent) -> Self {
        Event::Named(value)
    }
}

impl From<AiaEvent> for Event {
    fn from(value: AiaEvent) -> Self {
        Event::Aia(value)
    }
}

impl From<OemEvent> for Event {
    fn from(value: OemEvent) -> Self {
        Event::Oem(value)
    }
}

impl From<UiaEvent> for Event {
    fn from(value: UiaEvent) -> Self {
        Event::Uia(value)
    }
}

impl From<UiaPropertyEvent> for Event {
    fn from(value: UiaPropertyEvent) -> Self {
        Event::UiaProperty(value)
    }
}

impl From<Event> for u32 {
    fn from(value: Event) -> Self {
        match value {
            Event::Named(inner) => inner.into(),
            Event::Aia(inner) => inner.into(),
            Event::Oem(inner) => inner.into(),
            Event::Uia(inner) => inner.into(),
            Event::UiaProperty(inner) => inner.into(),
            Event::Unknown(value) => value,
        }
    }
}

impl From<&Event> for u32 {
    fn from(value: &Event) -> Self {
        match value {
            Event::Named(inner) => inner.into(),
            Event::Aia(inner) => inner.into(),
            Event::Oem(inner) => inner.into(),
            Event::Uia(inner) => inner.into(),
            Event::UiaProperty(inner) => inner.into(),
            Event::Unknown(value) => *value,
        }
    }
}

impl From<u32> for Event {
    fn from(value: u32) -> Self {
        if let Ok(event) = UiaPropertyEvent::try_from(value) {
            return Event::UiaProperty(event);
        }
        if let Ok(event) = UiaEvent::try_from(value) {
            return Event::Uia(event);
        }
        if let Ok(event) = OemEvent::try_from(value) {
            return Event::Oem(event);
        }
        if let Ok(event) = AiaEvent::try_from(value) {
            return Event::Aia(event);
        }
        if let Ok(event) = NamedEvent::try_from(value) {
            return Event::Named(event);
        }

        Event::Unknown(value)
    }
}

u32_to_enum! {
    /// Windows accessibility named event values.
    /// See [Event Constants](https://learn.microsoft.com/en-us/windows/win32/winauto/event-constants)
    /// for more information.
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    #[repr(u32)]
    #[non_exhaustive]
    pub enum NamedEvent {
        /// An object's KeyboardShortcut property has changed. Server applications send this event for their accessible objects.
        ObjectAcceleratorChange = EVENT_OBJECT_ACCELERATORCHANGE,
        /// Sent when a window is cloaked. A cloaked window still exists, but is invisible to the user.
        ObjectCloaked = EVENT_OBJECT_CLOAKED,
        /// A window object's scrolling has ended. Unlike EVENT_SYSTEM_SCROLLEND, this event is associated with the scrolling window. Whether the scrolling is horizontal or vertical scrolling, this event should be sent whenever the scroll action is completed.
        /// The hwnd parameter of the WinEventProc callback function describes the scrolling window; the idObject parameter is
        /// OBJID_CLIENT, and the idChild parameter is CHILDID_SELF.
        ObjectContentsScrolled = EVENT_OBJECT_CONTENTSCROLLED,
        /// An object has been created. The system sends this event for the following user interface elements: caret, header control, list-view control, tab control, toolbar control, tree view control, and window object. Server applications send this event for their accessible objects.
        /// Before sending the event for the parent object, servers must send it for all of an object's child objects. Servers must ensure that all child objects are fully created and ready to accept IAccessible calls from clients before the parent object sends this event.
        /// Because a parent object is created after its child objects, clients must make sure that an object's parent has been created before calling IAccessible::get_accParent, particularly if in-context hook functions are used.
        ObjectCreate = EVENT_OBJECT_CREATE,
        /// An object's DefaultAction property has changed. The system sends this event for dialog boxes. Server applications send this event for their accessible objects.
        ObjectDefactionChange = EVENT_OBJECT_DEFACTIONCHANGE,
        /// An object's Description property has changed. Server applications send this event for their accessible objects.
        ObjectDescriptionChange = EVENT_OBJECT_DESCRIPTIONCHANGE,
        /// An object has been destroyed. The system sends this event for the following user interface elements: caret, header control, list-view control, tab control, toolbar control, tree view control, and window object. Server applications send this event for their accessible objects.
        /// Clients assume that all of an object's children are destroyed when the parent object sends this event.
        /// After receiving this event, clients do not call an object's IAccessible properties or methods. However, the interface pointer must remain valid as long as there is a reference count on it (due to COM rules), but the UI element may no longer be present. Further calls on the interface pointer may return failure errors; to prevent this, servers create proxy objects and monitor their life spans.
        ObjectDestroy = EVENT_OBJECT_DESTROY,
        /// The user started to drag an element. The hwnd, idObject, and idChild parameters of the WinEventProc callback function identify the object being dragged.
        ObjectDragStart = EVENT_OBJECT_DRAGSTART,
        /// The user has ended a drag operation before dropping the dragged element on a drop target. The hwnd, idObject, and idChild parameters of the WinEventProc callback function identify the object being dragged.
        ObjectDragCancel = EVENT_OBJECT_DRAGCANCEL,
        /// The user dropped an element on a drop target. The hwnd, idObject, and idChild parameters of the WinEventProc callback function identify the object being dragged.
        ObjectDragComplete = EVENT_OBJECT_DRAGCOMPLETE,
        /// The user dragged an element into a drop target's boundary. The hwnd, idObject, and idChild parameters of the WinEventProc callback function identify the drop target.
        ObjectDragEnter = EVENT_OBJECT_DRAGENTER,
        /// The user dragged an element out of a drop target's boundary. The hwnd, idObject, and idChild parameters of the WinEventProc callback function identify the drop target.
        ObjectDragLeave = EVENT_OBJECT_DRAGLEAVE,
        /// The user dropped an element on a drop target. The hwnd, idObject, and idChild parameters of the WinEventProc callback function identify the drop target.
        ObjectDragDropped = EVENT_OBJECT_DRAGDROPPED,
        /// An object has received the keyboard focus. The system sends this event for the following user interface elements: list-view control, menu bar, pop-up menu, switch window, tab control, tree view control, and window object. Server applications send this event for their accessible objects.
        /// The hwnd parameter of the WinEventProc callback function identifies the window that receives the keyboard focus.
        ObjectFocus = EVENT_OBJECT_FOCUS,
        /// An object's Help property has changed. Server applications send this event for their accessible objects.
        ObjectHelpChange = EVENT_OBJECT_HELPCHANGE,
        /// An object is hidden. The system sends this event for the following user interface elements: caret and cursor. Server applications send this event for their accessible objects.
        /// When this event is generated for a parent object, all child objects are already hidden. Server applications do not send this event for the child objects.
        /// Hidden objects include the STATE_SYSTEM_INVISIBLE flag; shown objects do not include this flag. The EVENT_OBJECT_HIDE event also indicates that the STATE_SYSTEM_INVISIBLE flag is set. Therefore, servers do not send the EVENT_STATE_CHANGE event in this case.
        ObjectHide = EVENT_OBJECT_HIDE,
        /// A window that hosts other accessible objects has changed the hosted objects. A client might need to query the host window to discover the new hosted objects, especially if the client has been monitoring events from the window. A hosted object is an object from an accessibility framework (MSAA or UI Automation) that is different from that of the host. Changes in hosted objects that are from the same framework as the host should be handed with the structural change events, such as EVENT_OBJECT_CREATE for MSAA. For more info see comments within winuser.h.
        ObjectHostedObjectsInvalidated = EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED,
        /// An IME window has become hidden.
        ObjectImeHide = EVENT_OBJECT_IME_HIDE,
        /// An IME window has become visible.
        ObjectImeShow = EVENT_OBJECT_IME_SHOW,
        /// The size or position of an IME window has changed.
        ObjectImeChange = EVENT_OBJECT_IME_CHANGE,
        /// An object has been invoked; for example, the user has clicked a button. This event is supported by common controls and is used by UI Automation.
        /// For this event, the hwnd, ID, and idChild parameters of the WinEventProc callback function identify the item that is invoked.
        ObjectInvoked = EVENT_OBJECT_INVOKED,
        /// An object that is part of a live region has changed. A live region is an area of an application that changes frequently and/or asynchronously.
        ObjectLiveRegionChanged = EVENT_OBJECT_LIVEREGIONCHANGED,
        /// An object has changed location, shape, or size. The system sends this event for the following user interface elements: caret and window objects. Server applications send this event for their accessible objects.
        /// This event is generated in response to a change in the top-level object within the object hierarchy; it is not generated for any children that the object might have. For example, if the user resizes a window, the system sends this notification for the window, but not for the menu bar, title bar, scroll bar, or other objects that have also changed.
        /// The system does not send this event for every non-floating child window when the parent moves. However, if an application explicitly resizes child windows as a result of resizing the parent window, the system sends multiple events for the resized children.
        /// If an object's State property is set to STATE_SYSTEM_FLOATING, the server sends EVENT_OBJECT_LOCATIONCHANGE whenever the object changes location. If an object does not have this state, servers only trigger this event when the object moves in relation to its parent. For this event notification, the idChild parameter of the WinEventProc callback function identifies the child object that has changed.
        ObjectLocationChange = EVENT_OBJECT_LOCATIONCHANGE,
        /// An object's Name property has changed. The system sends this event for the following user interface elements: check box, cursor, list-view control, push button, radio button, status bar control, tree view control, and window object. Server applications send this event for their accessible objects.
        ObjectNameChange = EVENT_OBJECT_NAMECHANGE,
        /// An object has a new parent object. Server applications send this event for their accessible objects.
        ObjectParentChange = EVENT_OBJECT_PARENTCHANGE,
        /// A container object has added, removed, or reordered its children. The system sends this event for the following user interface elements: header control, list-view control, toolbar control, and window object. Server applications send this event as appropriate for their accessible objects.
        /// For example, this event is generated by a list-view object when the number of child elements or the order of the elements changes. This event is also sent by a parent window when the Z-order for the child windows changes.
        ObjectReorder = EVENT_OBJECT_REORDER,
        /// The selection within a container object has changed. The system sends this event for the following user interface elements: list-view control, tab control, tree view control, and window object. Server applications send this event for their accessible objects.
        /// This event signals a single selection: either a child is selected in a container that previously did not contain any selected children, or the selection has changed from one child to another.
        /// The hwnd and idObject parameters of the WinEventProc callback function describe the container; the idChild parameter identifies the object that is selected. If the selected child is a window that also contains objects, the idChild parameter is OBJID_WINDOW.
        ObjectSelection = EVENT_OBJECT_SELECTION,
        /// A child within a container object has been added to an existing selection. The system sends this event for the following user interface elements: list box, list-view control, and tree view control. Server applications send this event for their accessible objects.
        /// The hwnd and idObject parameters of the WinEventProc callback function describe the container. The idChild parameter is the child that is added to the selection.
        ObjectSelectionAdd = EVENT_OBJECT_SELECTIONADD,
        /// An item within a container object has been removed from the selection. The system sends this event for the following user interface elements: list box, list-view control, and tree view control. Server applications send this event for their accessible objects.
        /// This event signals that a child is removed from an existing selection.
        /// The hwnd and idObject parameters of the WinEventProc callback function describe the container; the idChild parameter identifies the child that has been removed from the selection.
        ObjectSelectionRemove = EVENT_OBJECT_SELECTIONREMOVE,
        /// Numerous selection changes have occurred within a container object. The system sends this event for list boxes; server applications send it for their accessible objects.
        /// This event is sent when the selected items within a control have changed substantially. The event informs the client that many selection changes have occurred, and it is sent instead of several EVENT_OBJECT_SELECTIONADD or EVENT_OBJECT_SELECTIONREMOVE events. The client queries for the selected items by calling the container object's IAccessible::get_accSelection method and enumerating the selected items.
        /// For this event notification, the hwnd and idObject parameters of the WinEventProc callback function describe the container in which the changes occurred.
        ObjectSelectionWithin = EVENT_OBJECT_SELECTIONWITHIN,
        /// A hidden object is shown. The system sends this event for the following user interface elements: caret, cursor, and window object. Server applications send this event for their accessible objects.
        /// Clients assume that when this event is sent by a parent object, all child objects are already displayed. Therefore, server applications do not send this event for the child objects.
        /// Hidden objects include the STATE_SYSTEM_INVISIBLE flag; shown objects do not include this flag. The EVENT_OBJECT_SHOW event also indicates that the STATE_SYSTEM_INVISIBLE flag is cleared. Therefore, servers do not send the EVENT_STATE_CHANGE event in this case.
        ObjectShow = EVENT_OBJECT_SHOW,
        /// An object's state has changed. The system sends this event for the following user interface elements: check box, combo box, header control, push button, radio button, scroll bar, toolbar control, tree view control, up-down control, and window object. Server applications send this event for their accessible objects.
        /// For example, a state change occurs when a button object is clicked or released, or when an object is enabled or disabled.
        /// For this event notification, the idChild parameter of the WinEventProc callback function identifies the child object whose state has changed.
        ObjectStateChange = EVENT_OBJECT_STATECHANGE,
        /// The conversion target within an IME composition has changed. The conversion target is the subset of the IME composition which is actively selected as the target for user-initiated conversions.
        ObjectConversionTargetChanged = EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED,
        /// An object's text selection has changed. This event is supported by common controls and is used by UI Automation.
        /// The hwnd, ID, and idChild parameters of the WinEventProc callback function describe the item that is contained in the updated text selection.
        ObjectTextSelectionChanged = EVENT_OBJECT_TEXTSELECTIONCHANGED,
        /// Sent when a window is uncloaked. A cloaked window still exists, but is invisible to the user.
        ObjectUncloaked = EVENT_OBJECT_UNCLOAKED,
        /// An object's Value property has changed. The system sends this event for the user interface elements that include the scroll bar and the following controls: edit, header, hot key, progress bar, slider, and up-down. Server applications send this event for their accessible objects.
        ObjectValueChange = EVENT_OBJECT_VALUECHANGE,
        /// An alert has been generated. Server applications should not send this event.
        SystemAlert = EVENT_SYSTEM_ALERT,
        /// A preview rectangle is being displayed.
        SystemArrangementPreview = EVENT_SYSTEM_ARRANGMENTPREVIEW,
        /// A window has lost mouse capture. This event is sent by the system, never by servers.
        SystemCaptureEnd = EVENT_SYSTEM_CAPTUREEND,
        /// A window has received mouse capture. This event is sent by the system, never by servers.
        SystemCaptureStart = EVENT_SYSTEM_CAPTURESTART,
        /// A window has exited context-sensitive Help mode. This event is not sent consistently by the system.
        SystemContextHelpEnd = EVENT_SYSTEM_CONTEXTHELPEND,
        /// A window has entered context-sensitive Help mode. This event is not sent consistently by the system.
        SystemContextHelpStart = EVENT_SYSTEM_CONTEXTHELPSTART,
        /// The active desktop has been switched.
        SystemDesktopSwitch = EVENT_SYSTEM_DESKTOPSWITCH,
        /// A dialog box has been closed. The system sends this event for standard dialog boxes; servers send it for custom dialog boxes. This event is not sent consistently by the system.
        SystemDialogEnd = EVENT_SYSTEM_DIALOGEND,
        /// A dialog box has been displayed. The system sends this event for standard dialog boxes, which are created using resource templates or Win32 dialog box functions. Servers send this event for custom dialog boxes, which are windows that function as dialog boxes but are not created in the standard way.
        /// This event is not sent consistently by the system.
        SystemDialogStart = EVENT_SYSTEM_DIALOGSTART,
        /// An application is about to exit drag-and-drop mode. Applications that support drag-and-drop operations must send this event; the system does not send this event.
        SystemDragDropEnd = EVENT_SYSTEM_DRAGDROPEND,
        /// An application is about to enter drag-and-drop mode. Applications that support drag-and-drop operations must send this event because the system does not send it.
        SystemDragDropStart = EVENT_SYSTEM_DRAGDROPSTART,
        /// The foreground window has changed. The system sends this event even if the foreground window has changed to another window in the same thread. Server applications never send this event.
        /// For this event, the WinEventProc callback function's hwnd parameter is the handle to the window that is in the foreground, the idObject parameter is OBJID_WINDOW, and the idChild parameter is CHILDID_SELF.
        SystemForeground = EVENT_SYSTEM_FOREGROUND,
        /// A pop-up menu has been closed. The system sends this event for standard menus; servers send it for custom menus.
        /// When a pop-up menu is closed, the client receives this message, and then the EVENT_SYSTEM_MENUEND event.
        /// This event is not sent consistently by the system.
        SystemMenuPopupEnd = EVENT_SYSTEM_MENUPOPUPEND,
        /// A pop-up menu has been displayed. The system sends this event for standard menus, which are identified by HMENU, and are created using menu-template resources or Win32 menu functions. Servers send this event for custom menus, which are user interface elements that function as menus but are not created in the standard way. This event is not sent consistently by the system.
        SystemMenuPopupStart = EVENT_SYSTEM_MENUPOPUPSTART,
        /// A menu from the menu bar has been closed. The system sends this event for standard menus; servers send it for custom menus.
        /// For this event, the WinEventProc callback function's hwnd, idObject, and idChild parameters refer to the control that contains the menu bar or the control that activates the context menu. The hwnd parameter is the handle to the window that is related to the event. The idObject parameter is OBJID_MENU or OBJID_SYSMENU for a menu, or OBJID_WINDOW for a pop-up menu. The idChild parameter is CHILDID_SELF.
        SystemMenuEnd = EVENT_SYSTEM_MENUEND,
        /// A menu item on the menu bar has been selected. The system sends this event for standard menus, which are identified by HMENU, created using menu-template resources or Win32 menu API elements. Servers send this event for custom menus, which are user interface elements that function as menus but are not created in the standard way.
        /// For this event, the WinEventProc callback function's hwnd, idObject, and idChild parameters refer to the control that contains the menu bar or the control that activates the context menu. The hwnd parameter is the handle to the window related to the event. The idObject parameter is OBJID_MENU or OBJID_SYSMENU for a menu, or OBJID_WINDOW for a pop-up menu. The idChild parameter is CHILDID_SELF.
        /// The system triggers more than one EVENT_SYSTEM_MENUSTART event that does not always correspond with the EVENT_SYSTEM_MENUEND event.
        SystemMenuStart = EVENT_SYSTEM_MENUSTART,
        /// A window object is about to be restored. This event is sent by the system, never by servers.
        SystemMinimizeEnd = EVENT_SYSTEM_MINIMIZEEND,
        /// A window object is about to be minimized. This event is sent by the system, never by servers.
        SystemMinimizeStart = EVENT_SYSTEM_MINIMIZESTART,
        /// The movement or resizing of a window has finished. This event is sent by the system, never by servers.
        SystemMoveSizeEnd = EVENT_SYSTEM_MOVESIZEEND,
        /// A window is being moved or resized. This event is sent by the system, never by servers.
        SystemMoveSizeStart = EVENT_SYSTEM_MOVESIZESTART,
        /// Scrolling has ended on a scroll bar. This event is sent by the system for standard scroll bar controls and for scroll bars that are attached to a window. Servers send this event for custom scroll bars, which are user interface elements that function as scroll bars but are not created in the standard way.
        /// The idObject parameter that is sent to the WinEventProc callback function is OBJID_HSCROLL for horizontal scroll bars, and OBJID_VSCROLL for vertical scroll bars.
        SystemScrollingEnd = EVENT_SYSTEM_SCROLLINGEND,
        /// Scrolling has started on a scroll bar. The system sends this event for standard scroll bar controls and for scroll bars attached to a window. Servers send this event for custom scroll bars, which are user interface elements that function as scroll bars but are not created in the standard way.
        /// The idObject parameter that is sent to the WinEventProc callback function is OBJID_HSCROLL for horizontal scrolls bars, and OBJID_VSCROLL for vertical scroll bars.
        SystemScrollingStart = EVENT_SYSTEM_SCROLLINGSTART,
        /// A sound has been played. The system sends this event when a system sound, such as one for a menu, is played even if no sound is audible (for example, due to the lack of a sound file or a sound card). Servers send this event whenever a custom UI element generates a sound.
        /// For this event, the WinEventProc callback function receives the OBJID_SOUND value as the idObject parameter.
        SystemSound = EVENT_SYSTEM_SOUND,
        /// The user has released ALT+TAB. This event is sent by the system, never by servers. The hwnd parameter of the WinEventProc callback function identifies the window to which the user has switched.
        /// If only one application is running when the user presses ALT+TAB, the system sends this event without a corresponding EVENT_SYSTEM_SWITCHSTART event.
        SystemSwitchEnd = EVENT_SYSTEM_SWITCHEND,
        /// The user has pressed ALT+TAB, which activates the switch window. This event is sent by the system, never by servers. The hwnd parameter of the WinEventProc callback function identifies the window to which the user is switching.
        /// If only one application is running when the user presses ALT+TAB, the system sends an EVENT_SYSTEM_SWITCHEND event without a corresponding EVENT_SYSTEM_SWITCHSTART event.
        SystemSwitchStart = EVENT_SYSTEM_SWITCHSTART,
    }
}

impl NamedEvent {
    /// The highest object event value.
    pub const OBJECT_END: u32 = EVENT_OBJECT_END;

    /// The highest system event value.
    pub const SYSTEM_END: u32 = EVENT_SYSTEM_END;

    /// Determines if a given [`u32`] can be represented as an [`NamedEvent`].
    pub fn is_within_range(value: u32) -> bool {
        // use the generated TryFrom<u32> implementation to check the value
        NamedEvent::try_from(value).is_ok()
    }

    /// Determines if the instance contains a valid value.
    pub fn is_valid(self) -> bool {
        Self::is_within_range(self.into())
    }
}

impl From<NamedEvent> for u32 {
    fn from(value: NamedEvent) -> Self {
        // just do a direct cast
        // this should always be safe as the enum is `repr(u32)`
        unsafe { std::mem::transmute(value) }
    }
}

impl From<&NamedEvent> for u32 {
    fn from(value: &NamedEvent) -> Self {
        // just do the clone to avoid more transmute logic
        value.to_owned().into()
    }
}

/// Windows accessibility event within the Accessibility Interoperability Alliance (AIA) range.
/// See [Community Reserved Events](https://learn.microsoft.com/en-us/windows/win32/winauto/allocation-of-winevent-ids#community-reserved-events)
/// for more information.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AiaEvent(u32);

impl AiaEvent {
    /// The lowest possible [`AiaEvent`] value.
    pub const MIN: u32 = EVENT_AIA_START;

    /// The highest possible [`AiaEvent`] value.
    pub const MAX: u32 = EVENT_AIA_END;

    /// Determines if a given [`u32`] is within the
    /// [Accessibility Interoperability Alliance](https://learn.microsoft.com/en-us/windows/win32/winauto/allocation-of-winevent-ids#community-reserved-events)
    /// reserved range.
    pub fn is_within_range(value: u32) -> bool {
        (AiaEvent::MIN..AiaEvent::MAX).contains(&value)
    }

    /// Determines if the instance contains a valid value.
    pub fn is_valid(self) -> bool {
        Self::is_within_range(self.into())
    }
}

impl TryFrom<u32> for AiaEvent {
    type Error = crate::errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if AiaEvent::is_within_range(value) {
            Ok(AiaEvent(value))
        } else {
            Err(crate::errors::Error::InvalidRangedEvent {
                event: value,
                min: AiaEvent::MIN,
                max: AiaEvent::MAX,
            })
        }
    }
}

impl From<AiaEvent> for u32 {
    fn from(value: AiaEvent) -> Self {
        value.0
    }
}

impl From<&AiaEvent> for u32 {
    fn from(value: &AiaEvent) -> Self {
        value.0
    }
}

/// Windows accessibility event within the OEM Reserved Event range.
/// See [OEM Reserved Events](https://learn.microsoft.com/en-us/windows/win32/winauto/allocation-of-winevent-ids#oem-reserved-events)
/// for more information.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct OemEvent(u32);

impl OemEvent {
    /// The lowest possible [`OemEvent`] value.
    pub const MIN: u32 = EVENT_OEM_DEFINED_START;

    /// The highest possible [`OemEvent`] value.
    pub const MAX: u32 = EVENT_OEM_DEFINED_END;

    /// Determines if a given [`u32`] is within the
    /// [OEM](https://learn.microsoft.com/en-us/windows/win32/winauto/allocation-of-winevent-ids#oem-reserved-events)
    /// reserved range.
    pub fn is_within_range(value: u32) -> bool {
        (OemEvent::MIN..OemEvent::MAX).contains(&value)
    }

    /// Determines if the instance contains a valid value.
    pub fn is_valid(self) -> bool {
        Self::is_within_range(self.into())
    }
}

impl TryFrom<u32> for OemEvent {
    type Error = crate::errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if OemEvent::is_within_range(value) {
            Ok(OemEvent(value))
        } else {
            Err(crate::errors::Error::InvalidRangedEvent {
                event: value,
                min: OemEvent::MIN,
                max: OemEvent::MAX,
            })
        }
    }
}

impl From<OemEvent> for u32 {
    fn from(value: OemEvent) -> Self {
        value.0
    }
}

impl From<&OemEvent> for u32 {
    fn from(value: &OemEvent) -> Self {
        value.0
    }
}

/// Windows accessibility event within the UI Automation Event range.
/// See [Microsoft Activity Accessibility and UI Automation Events](https://learn.microsoft.com/en-us/windows/win32/winauto/allocation-of-winevent-ids#microsoft-active-accessibility-and-ui-automation-events)
/// for more information.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct UiaEvent(u32);

impl UiaEvent {
    /// The lowest possible [`UiaEvent`] value.
    pub const MIN: u32 = EVENT_UIA_EVENTID_START;

    /// The highest possible [`UiaEvent`] value.
    pub const MAX: u32 = EVENT_UIA_EVENTID_END;

    /// Determines if a given [`u32`] is within the
    /// [UI Automation](https://learn.microsoft.com/en-us/windows/win32/winauto/allocation-of-winevent-ids#microsoft-active-accessibility-and-ui-automation-events)
    /// event reserved range.
    pub fn is_within_range(value: u32) -> bool {
        (UiaEvent::MIN..UiaEvent::MAX).contains(&value)
    }

    /// Determines if the instance contains a valid value.
    pub fn is_valid(self) -> bool {
        Self::is_within_range(self.into())
    }
}

impl TryFrom<u32> for UiaEvent {
    type Error = crate::errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if UiaEvent::is_within_range(value) {
            Ok(UiaEvent(value))
        } else {
            Err(crate::errors::Error::InvalidRangedEvent {
                event: value,
                min: UiaEvent::MIN,
                max: UiaEvent::MAX,
            })
        }
    }
}

impl From<UiaEvent> for u32 {
    fn from(value: UiaEvent) -> Self {
        value.0
    }
}

impl From<&UiaEvent> for u32 {
    fn from(value: &UiaEvent) -> Self {
        value.0
    }
}

/// Windows accessibility event within the UI Automation Property Change Event range.
/// See [Microsoft Activity Accessibility and UI Automation Events](https://learn.microsoft.com/en-us/windows/win32/winauto/allocation-of-winevent-ids#microsoft-active-accessibility-and-ui-automation-events)
/// for more information.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct UiaPropertyEvent(u32);

impl UiaPropertyEvent {
    /// The lowest possible [`UiaPropertyEvent`] value.
    pub const MIN: u32 = EVENT_UIA_PROPID_START;

    /// The highest possible [`UiaPropertyEvent`] value.
    pub const MAX: u32 = EVENT_UIA_PROPID_END;

    /// Determines if a given [`u32`] is within the
    /// [UI Automation](https://learn.microsoft.com/en-us/windows/win32/winauto/allocation-of-winevent-ids#microsoft-active-accessibility-and-ui-automation-events)
    /// property-changed event reserved range.
    pub fn is_within_range(value: u32) -> bool {
        (UiaPropertyEvent::MIN..UiaPropertyEvent::MAX).contains(&value)
    }

    /// Determines if the instance contains a valid value.
    pub fn is_valid(self) -> bool {
        Self::is_within_range(self.into())
    }
}

impl TryFrom<u32> for UiaPropertyEvent {
    type Error = crate::errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if UiaPropertyEvent::is_within_range(value) {
            Ok(UiaPropertyEvent(value))
        } else {
            Err(crate::errors::Error::InvalidRangedEvent {
                event: value,
                min: UiaPropertyEvent::MIN,
                max: UiaPropertyEvent::MAX,
            })
        }
    }
}

impl From<UiaPropertyEvent> for u32 {
    fn from(value: UiaPropertyEvent) -> Self {
        value.0
    }
}

impl From<&UiaPropertyEvent> for u32 {
    fn from(value: &UiaPropertyEvent) -> Self {
        value.0
    }
}
