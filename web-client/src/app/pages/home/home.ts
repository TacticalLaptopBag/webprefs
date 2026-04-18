import { Component, inject, OnInit } from '@angular/core';
import { LogoutButton } from "../../logout-button/logout-button";
import { Prefs } from "./prefs/prefs";
import { Scopes } from "./scopes/scopes";
import { ActivatedRoute, Router } from '@angular/router';

@Component({
    selector: 'app-home',
    imports: [LogoutButton, Prefs, Scopes],
    templateUrl: './home.html',
    styleUrl: './home.css',
})
export class Home implements OnInit {
    public scope: string | null = null

    private _route = inject(ActivatedRoute)

    ngOnInit(): void {
        this.scope = this._route.snapshot.paramMap.get('scope')
    }
}
