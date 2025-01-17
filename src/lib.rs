use gloo::console::log;
use obsidian_rs::wasm;
use obsidian_rs::Notice;
use obsidian_rs::Plugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn onload(raw_plugin: wasm::Plugin) {
    log!("Loaded");

    let plugin = Plugin::from(raw_plugin);

    // This creates an icon in the left ribbon.
    let ribbon_icon_el = plugin.add_ribbon_icon("dice", "Sample Plugin", || {
        {
            // Called when the user clicks the icon.
            Notice::new("This is a notice!");
        }
    });
    // Perform additional things with the ribbon
    ribbon_icon_el
        .class_list()
        .add_1("my-plugin-ribbon-class")
        .unwrap();

    // This adds a status bar item to the bottom of the app. Does not work on mobile apps.
    let status_bar_item_el = plugin.add_status_bar_item();
    status_bar_item_el.set_text_content(Some("Status Bar Text"));

    // This adds a simple command that can be triggered anywhere
    plugin.add_command(
        "open-sample-modal-simple",
        "Open sample modal (simple)",
        || {
            // 		new SampleModal(this.app).open();
            Notice::new("COmmand");
        },
    );

    // this.addCommand({
    // 	id: 'open-sample-modal-simple',
    // 	name: 'Open sample modal (simple)',
    // 	callback: () => {
    // 		new SampleModal(this.app).open();
    // 	}
    // });
    // // This adds an editor command that can perform some operation on the current editor instance
    // this.addCommand({
    // 	id: 'sample-editor-command',
    // 	name: 'Sample editor command',
    // 	editorCallback: (editor: Editor, view: MarkdownView) => {
    // 		console.log(editor.getSelection());
    // 		editor.replaceSelection('Sample Editor Command');
    // 	}
    // });
    // // This adds a complex command that can check whether the current state of the app allows execution of the command
    // this.addCommand({
    // 	id: 'open-sample-modal-complex',
    // 	name: 'Open sample modal (complex)',
    // 	checkCallback: (checking: boolean) => {
    // 		// Conditions to check
    // 		const markdownView = this.app.workspace.getActiveViewOfType(MarkdownView);
    // 		if (markdownView) {
    // 			// If checking is true, we're simply "checking" if the command can be run.
    // 			// If checking is false, then we want to actually perform the operation.
    // 			if (!checking) {
    // 				new SampleModal(this.app).open();
    // 			}

    // 			// This command will only show up in Command Palette when the check function returns true
    // 			return true;
    // 		}
    // 	}
    // });

    // // This adds a settings tab so the user can configure various aspects of the plugin
    // this.addSettingTab(new SampleSettingTab(this.app, this));

    // // If the plugin hooks up any global DOM events (on parts of the app that doesn't belong to this plugin)
    // // Using this function will automatically remove the event listener when this plugin is disabled.
    // this.registerDomEvent(document, 'click', (evt: MouseEvent) => {
    // 	console.log('click', evt);
    // });

    // // When registering intervals, this function will automatically clear the interval when the plugin is disabled.
    // this.registerInterval(window.setInterval(() => console.log('setInterval'), 5 * 60 * 1000));}
}
