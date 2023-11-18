import React from 'react'
import Posts from './views/thulir/news/posts'
import Post from './views/thulir/news/post'

const Dashboard = React.lazy(() => import('./views/dashboard/Dashboard'))

const routes = [
  { path: '/', exact: true, name: 'Home' },
  { path: '/dashboard', name: 'Dashboard', element: Dashboard },
  
  { path: '/news', name: 'News', element: Posts },
  { path: '/post/:postId', name: 'post', element: Post },
]

export default routes
