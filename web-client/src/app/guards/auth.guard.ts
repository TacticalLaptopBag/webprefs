import { inject, Injectable } from '@angular/core';
import { CanActivate, Router } from '@angular/router';
import { Observable, catchError, map, of, switchMap, take } from 'rxjs';
import { AuthService } from '../services/auth.service';

@Injectable({ providedIn: 'root' })
export class AuthGuard implements CanActivate {
    private _authService = inject(AuthService)
    private _router = inject(Router)

    canActivate(): Observable<boolean> {
        return this._authService.checkLogin().pipe(
            map(() => true),
            catchError(() => {
                // Check login failed — try refreshing the token first
                return this._authService.refresh().pipe(
                    map(() => true),
                    catchError(() => {
                        // Refresh also failed — redirect to login
                        this._router.navigate(['/login']);
                        return of(false);
                    })
                );
            })
        );
    }
}
