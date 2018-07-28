import React, {
  Component
} from 'react'
import {
  FormattedMessage,
  intlShape,
  injectIntl
} from 'react-intl'
import router from 'umi/router'
import {
  Button,
  Popconfirm,
  message
} from 'antd';

import Layout from "../../../components/layouts/Table"
import Authorized from '../../../components/Authorized'
import {
  failed,
  client
} from '../../../utils/request'
import {
  is_administrator
} from '../../../utils/authorized'

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [],
      columns: [{
        title: (<FormattedMessage id="attributes.position"/>),
        dataIndex: 'position',
        width: 80,
        key: 'position'
      }, {
        title: (<FormattedMessage id="attributes.url"/>),
        render: (text, record) => <a target="_blank" rel="noopener noreferrer" href={record.home}>{record.title}</a>,
        key: 'user'
      }, {
        title: (<span><FormattedMessage id="buttons.operator"/>&nbsp;<Button onClick={()=>router.push(`/admin/friend-links/new`)} size="small" shape="circle" type="primary" icon="file-add"/></span>),
        width: 120,
        render: (text, record) => (<span>
          <Button size="small" onClick={()=>router.push(`/admin/friend-links/${record.id}/edit`)} shape="circle" type="dashed" icon="edit"/>
          <Popconfirm title={<FormattedMessage id="flashes.confirm-to-remove" values={{name:record.title}} />} onConfirm={(e) => this.handleRemove(record.id)}>
            <Button size="small" shape="circle" type="danger" icon="delete"/>
          </Popconfirm>
        </span>),
        key: 'operator'
      }]
    }
  }
  handleRemove = (id) => {
    const {
      formatMessage
    } = this.props.intl
    client().request(`mutation form($id: String!){
          removeFriendLink(id: $id) {
            createdAt
          }
        }`, {
      id
    }).then(() => {
      message.info(formatMessage({
        id: "flashes.success"
      }))
      var items = this.state.items.filter((it) => it.id !== id)
      this.setState({
        items
      })
    }).catch(failed)
  }
  componentDidMount() {
    client().request(`query list{
        listFriendLink{
          id, title, home, position
        }
      }`).then((rst) => {
      this.setState({
        items: rst.listFriendLink
      })
    }).catch(failed)
  }
  render() {
    const {
      items,
      columns
    } = this.state
    return (<Authorized check={is_administrator}>
      <Layout rowKey="id" title={{
        id: "nut.admin.friend-links.index.title"
      }} items={items} columns={columns}/>
    </Authorized>)
  }
}


Widget.propTypes = {
  intl: intlShape.isRequired
}

export default injectIntl(Widget)