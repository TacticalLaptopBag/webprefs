import { Component, inject, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AuthService } from '../../../services/auth.service';

@Component({
    selector: 'app-login',
    imports: [FormsModule],
    templateUrl: './login.html',
    styleUrl: './login.css',
})
export class Login {
    public errorMsg = signal<string | null>(null)
    public username = signal('')
    public password = signal('')
    public isLoading = signal(false)

    private _authSvc = inject(AuthService)

    public onSubmit() {
        this.isLoading.set(true)
        this.errorMsg.set(null)
        this._authSvc.login(this.username(), this.password()).subscribe({
            error: (err) => {
                this.errorMsg.set(err?.error?.error ?? err?.error ?? err)
                this.isLoading.set(false)
            }
        })
    }
}
