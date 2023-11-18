import {
    CButton,
    CCard,
    CCardBody,
    CCardHeader,
    CCol,
    CRow,
    CCardTitle,
    CCardImage,
    CCardText
} from '@coreui/react'

function PostSummary({ post }) {
    function renderLink(){
        const link = `#post/${post.id}`
        return <a href={link}>View</a>
    }
    return (
        <div>
            <CCard>
                <CCardBody>
                    <CCardTitle>{post.title.rendered}</CCardTitle>
                    <CCardText>
                        {post.excerpt.rendered}
                    </CCardText>
                </CCardBody>
                {renderLink()}
            </CCard>
        </div>
    );
}

export default PostSummary;