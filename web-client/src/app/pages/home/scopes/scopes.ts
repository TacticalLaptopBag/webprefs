import { Component, inject } from '@angular/core';
import { RouterLink } from "@angular/router";
import { PrefsService } from '../../../services/prefs.service';
import { AsyncPipe } from '@angular/common';

@Component({
    selector: 'app-scopes',
    imports: [RouterLink, AsyncPipe],
    templateUrl: './scopes.html',
    styleUrl: './scopes.css',
})
export class Scopes {
    private _prefsSvc = inject(PrefsService)
    public scopes$ = this._prefsSvc.getScopes()
}
