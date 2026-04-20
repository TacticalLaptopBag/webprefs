import { Component, Input, OnInit, inject, signal } from '@angular/core';
import { PrefsService } from '../../../services/prefs.service';
import { AsyncPipe } from '@angular/common';
import { Observable, of } from 'rxjs';
import { PrefEntry } from '../../../models/pref-entry.interface';
import { FormsModule } from '@angular/forms';
import { RouterLink } from "@angular/router";

@Component({
    selector: 'app-prefs',
    imports: [AsyncPipe, FormsModule, RouterLink],
    templateUrl: './prefs.html',
    styleUrl: './prefs.css',
})
export class Prefs implements OnInit {
    @Input()
    public scope!: string

    private _prefSvc = inject(PrefsService)
    public prefs$ = signal<Observable<PrefEntry[]>>(of([]))

    public newEntryKey = ''
    public newEntryValue = ''

    ngOnInit(): void {
        this.prefs$.set(this._prefSvc.getPrefsInScope(this.scope))
    }

    private refresh() {
        this.prefs$.set(this._prefSvc.getPrefsInScope(this.scope))
    }

    public submitChange(entry: PrefEntry) {
        this._prefSvc.updatePref(entry.pref_scope, entry.pref_key, entry.pref_value).subscribe(() => {
            this.refresh()
        })
    }

    public deletePref(entry: PrefEntry) {
        this._prefSvc.deletePref(entry.pref_scope, entry.pref_key).subscribe(() => {
            this.refresh()
        })
    }

    public createEntry() {
        this._prefSvc.createPref(this.scope, this.newEntryKey, this.newEntryValue).subscribe(() => {
            this.refresh()
            this.newEntryKey = ''
            this.newEntryValue = ''
        })
    }
}
