import { PrefEntry } from "./pref-entry.interface"

export interface ScopesResponseData {
    scopes: string[]
}

export interface PrefsResponseData {
    prefs: PrefEntry[]
}

export interface PrefValueResponseData {
    value: string | null
}
