import { Component, inject, OnInit, signal } from '@angular/core';
import { RouterLink } from "@angular/router";
import { PrefsService } from '../../../services/prefs.service';
import { AsyncPipe } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Observable, of } from 'rxjs';

@Component({
    selector: 'app-scopes',
    imports: [RouterLink, AsyncPipe, FormsModule],
    templateUrl: './scopes.html',
    styleUrl: './scopes.css',
})
export class Scopes implements OnInit {
    public scopes$ = signal<Observable<string[]>>(of([]))

    public newEntryScope = ''
    public newEntryKey = ''
    public newEntryValue = ''

    private _prefsSvc = inject(PrefsService)

    // TODO: The entry creator should be a separate component
    ngOnInit(): void {
        this.refresh()
    }

    private refresh() {
        this.scopes$.set(this._prefsSvc.getScopes())
    }

    public createEntry() {
        this._prefsSvc.createPref(this.newEntryScope, this.newEntryKey, this.newEntryValue).subscribe(() => {
            this.newEntryScope = ''
            this.newEntryKey = ''
            this.newEntryValue = ''
            this.refresh()
        })
    }
}
