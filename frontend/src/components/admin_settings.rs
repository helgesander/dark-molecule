use yew::prelude::*;
use crate::debug_log;

#[function_component(AdminSettings)]
pub fn admin_settings() -> Html {
    html! {
        <div class="admin-settings">
            <div class="section-header">
                <h2>{"System Settings"}</h2>
            </div>
            <div class="settings-grid">
                <div class="settings-card">
                    <h3>{"General Settings"}</h3>
                    <div class="settings-group">
                        <label>
                            <span>{"System Name"}</span>
                            <input type="text" value="Dark Molecule" />
                        </label>
                        <label>
                            <span>{"System Description"}</span>
                            <textarea>{"Security assessment and vulnerability management system"}</textarea>
                        </label>
                    </div>
                </div>
                <div class="settings-card">
                    <h3>{"Security Settings"}</h3>
                    <div class="settings-group">
                        <label>
                            <span>{"Session Timeout (minutes)"}</span>
                            <input type="number" value="30" />
                        </label>
                        <label>
                            <span>{"Password Policy"}</span>
                            <select>
                                <option value="standard">{"Standard"}</option>
                                <option value="strict">{"Strict"}</option>
                                <option value="custom">{"Custom"}</option>
                            </select>
                        </label>
                    </div>
                </div>
                <div class="settings-card">
                    <h3>{"Email Settings"}</h3>
                    <div class="settings-group">
                        <label>
                            <span>{"SMTP Server"}</span>
                            <input type="text" value="smtp.example.com" />
                        </label>
                        <label>
                            <span>{"SMTP Port"}</span>
                            <input type="number" value="587" />
                        </label>
                        <label>
                            <span>{"From Email"}</span>
                            <input type="email" value="noreply@example.com" />
                        </label>
                    </div>
                </div>
            </div>
            <div class="settings-actions">
                <button class="save-button">{"Save Changes"}</button>
            </div>
        </div>
    }
} 