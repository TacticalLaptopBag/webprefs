import { Routes } from '@angular/router';
import { LoginPage } from './pages/login-page/login-page';
import { AuthGuard } from './guards/auth.guard';
import { Home } from './pages/home/home';

export const routes: Routes = [
    {
        path: '',
        component: Home,
        canActivate: [AuthGuard],
    },
    {
        path: 'scope/:scope',
        component: Home,
        canActivate: [AuthGuard],
    },
    {
        path: 'login',
        component: LoginPage,
    }
];
