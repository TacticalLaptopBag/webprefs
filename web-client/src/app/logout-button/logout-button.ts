import { HttpClient } from '@angular/common/http';
import { Component, inject, signal } from '@angular/core';
import { environment } from '../../environments/environment';
import { AuthService } from '../services/auth.service';

@Component({
    selector: 'app-logout-button',
    imports: [],
    templateUrl: './logout-button.html',
    styleUrl: './logout-button.css',
})
export class LogoutButton {
    public isLoading = signal(false)
    private _authSvc = inject(AuthService)

    public onClick() {
        this.isLoading.set(true)
        this._authSvc.logout().subscribe({
            error: () => this.isLoading.set(false)
        })
    }
}
